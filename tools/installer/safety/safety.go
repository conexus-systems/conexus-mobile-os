// Package safety reúne as checagens de segurança do instalador (sem rede, sem adb/fastboot).
// Espelha scripts/installer_safety.py: os mesmos casos de teste valem para as duas versões.
// Regra geral: na dúvida, recusar (fail closed).
package safety

import (
	"archive/zip"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
	"os"
	"path"
	"path/filepath"
	"regexp"
	"strings"
)

// RequiredImages são as imagens sem as quais nada é gravado: pacote sem elas é recusado.
var RequiredImages = []string{"boot.img", "vbmeta.img", "super.img"}

const (
	maxEntries       = 64
	DefaultMaxTotal  = int64(16) << 30
	maxEntryReadSize = int64(16) << 30
)

var sha256Re = regexp.MustCompile(`^[0-9a-fA-F]{64}$`)

// ErrUnsafe indica entrada recusada por não passar nas checagens de segurança.
var ErrUnsafe = errors.New("recusado pela checagem de segurança")

func unsafef(format string, a ...any) error {
	return fmt.Errorf("%w: %s", ErrUnsafe, fmt.Sprintf(format, a...))
}

// IsValidSHA256 informa se s tem exatamente 64 dígitos hexadecimais.
func IsValidSHA256(s string) bool { return sha256Re.MatchString(s) }

// ParseSHA256Sums lê um SHA256SUMS (`<hash>  <arquivo>`) e devolve {arquivo: hash em minúsculas}.
// Linhas inválidas e hashes que não têm 64 hex são ignorados.
func ParseSHA256Sums(text string) map[string]string {
	sums := map[string]string{}
	for _, line := range strings.Split(text, "\n") {
		fields := strings.Fields(line)
		if len(fields) != 2 {
			continue
		}
		name := strings.TrimSpace(strings.TrimPrefix(fields[1], "*"))
		if IsValidSHA256(fields[0]) && name != "" {
			sums[name] = strings.ToLower(fields[0])
		}
	}
	return sums
}

// SHA256File calcula o SHA-256 de um arquivo.
func SHA256File(p string) (string, error) {
	f, err := os.Open(p)
	if err != nil {
		return "", err
	}
	defer f.Close()
	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return "", err
	}
	return hex.EncodeToString(h.Sum(nil)), nil
}

// VerifySHA256 recusa se o hash esperado for inválido ou diferente do calculado.
func VerifySHA256(p, expected string) error {
	if !IsValidSHA256(expected) {
		return unsafef("SHA-256 esperado ausente ou inválido; recusando instalar sem verificação")
	}
	actual, err := SHA256File(p)
	if err != nil {
		return err
	}
	if actual != strings.ToLower(expected) {
		return unsafef("SHA-256 não confere: esperado %s, obtido %s", strings.ToLower(expected), actual)
	}
	return nil
}

// SafeExtract extrai só arquivos .img/.txt/.json no nível raiz de dest. Recusa caminho absoluto,
// "..", subpastas, contrabarra, links simbólicos, excesso de entradas e tamanho acima de maxTotal.
func SafeExtract(zipPath, dest string, maxTotal int64) ([]string, error) {
	zr, err := zip.OpenReader(zipPath)
	if err != nil {
		return nil, err
	}
	defer zr.Close()

	if len(zr.File) > maxEntries {
		return nil, unsafef("pacote com %d entradas (máximo %d)", len(zr.File), maxEntries)
	}
	var total uint64
	for _, f := range zr.File {
		total += f.UncompressedSize64
	}
	if maxTotal > 0 && total > uint64(maxTotal) {
		return nil, unsafef("tamanho descompactado acima do limite")
	}
	if err := os.MkdirAll(dest, 0o755); err != nil {
		return nil, err
	}

	var out []string
	for _, f := range zr.File {
		name := f.Name
		if f.FileInfo().IsDir() {
			continue
		}
		if strings.HasPrefix(name, "/") || strings.Contains(name, "\\") || strings.Contains(name, "/") ||
			path.Clean(name) != name || name == ".." || strings.HasPrefix(name, "../") {
			return nil, unsafef("entrada com caminho recusado: %q", name)
		}
		if f.Mode()&os.ModeSymlink != 0 {
			return nil, unsafef("link simbólico recusado: %q", name)
		}
		if !(strings.HasSuffix(name, ".img") || strings.HasSuffix(name, ".txt") || strings.HasSuffix(name, ".json")) {
			return nil, unsafef("tipo de arquivo recusado: %q", name)
		}
		target := filepath.Join(dest, name)
		if err := extractOne(f, target); err != nil {
			return nil, err
		}
		out = append(out, target)
	}
	return out, nil
}

func extractOne(f *zip.File, target string) error {
	rc, err := f.Open()
	if err != nil {
		return err
	}
	defer rc.Close()
	w, err := os.OpenFile(target, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o644)
	if err != nil {
		return err
	}
	defer w.Close()
	// O tamanho declarado no cabeçalho pode mentir: limita a leitura real.
	n, err := io.Copy(w, io.LimitReader(rc, int64(f.UncompressedSize64)+1))
	if err != nil {
		return err
	}
	if n > int64(f.UncompressedSize64) {
		return unsafef("entrada %q maior que o tamanho declarado", f.Name)
	}
	return nil
}

// RequireImages recusa se faltar qualquer imagem obrigatória entre os arquivos extraídos.
func RequireImages(extracted []string) error {
	have := map[string]bool{}
	for _, p := range extracted {
		have[filepath.Base(p)] = true
	}
	var missing []string
	for _, r := range RequiredImages {
		if !have[r] {
			missing = append(missing, r)
		}
	}
	if len(missing) > 0 {
		return unsafef("imagens obrigatórias ausentes no pacote: %s", strings.Join(missing, ", "))
	}
	return nil
}

// ConfirmationPhrase é a frase que a pessoa digita para autorizar o flash deste aparelho.
func ConfirmationPhrase(serial, codename string) string {
	return "FLASH " + codename + " " + serial
}
