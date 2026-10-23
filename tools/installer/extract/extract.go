// Package extract copia do aparelho conectado os arquivos de vendor de que o módulo de drivers
// depende (módulos de kernel, HALs, firmware, scripts de init), para uso na construção do módulo
// do PRÓPRIO aparelho. É o equivalente ao extract-files do LineageOS.
//
// Só lê do aparelho (adb pull). Só copia caminhos de uma lista fechada. Grava um manifesto com
// SHA-256 e um .gitignore que impede commitar o conteúdo: os arquivos são do fabricante e
// NÃO podem ser redistribuídos pelo projeto (ADR-014).
package extract

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"time"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
)

// Roots é a lista fechada do que pode ser copiado.
var Roots = []string{
	"/vendor/lib/modules",
	"/vendor/lib64/modules",
	"/vendor/lib/hw",
	"/vendor/lib64/hw",
	"/vendor/bin/hw",
	"/vendor/firmware",
	"/vendor/etc/init",
	"/vendor/etc/vintf",
}

// File é um arquivo copiado.
type File struct {
	Path   string `json:"path"`
	Size   int64  `json:"size"`
	SHA256 string `json:"sha256"`
}

// Manifest descreve o que foi copiado. Não contém serial nem IMEI.
type Manifest struct {
	When    string   `json:"when"`
	Model   string   `json:"model,omitempty"`
	Device  string   `json:"device,omitempty"`
	Build   string   `json:"build_fingerprint,omitempty"`
	Files   []File   `json:"files"`
	Skipped []string `json:"skipped_roots,omitempty"`
}

const notice = `ESTES ARQUIVOS SÃO DO FABRICANTE DO APARELHO.
Foram copiados do seu próprio aparelho para uso pessoal na construção do módulo de drivers dele.
NÃO os publique, NÃO os envie ao repositório do projeto e NÃO os inclua em releases.
O .gitignore desta pasta impede o commit por engano; não o remova.
`

// Run copia as raízes permitidas para outDir. outDir precisa não existir ou estar vazio.
func Run(r device.Runner, serial, outDir string) (*Manifest, error) {
	if entries, err := os.ReadDir(outDir); err == nil && len(entries) > 0 {
		return nil, fmt.Errorf("%s não está vazio; use uma pasta nova", outDir)
	} else if err != nil && !errors.Is(err, fs.ErrNotExist) {
		return nil, err
	}
	if err := os.MkdirAll(outDir, 0o700); err != nil {
		return nil, err
	}
	if err := os.WriteFile(filepath.Join(outDir, ".gitignore"), []byte("*\n"), 0o600); err != nil {
		return nil, err
	}
	if err := os.WriteFile(filepath.Join(outDir, "AVISO.txt"), []byte(notice), 0o600); err != nil {
		return nil, err
	}

	m := &Manifest{When: time.Now().UTC().Format(time.RFC3339)}
	m.Model = prop(r, serial, "ro.product.model")
	m.Device = prop(r, serial, "ro.product.device")
	m.Build = prop(r, serial, "ro.build.fingerprint")

	for _, root := range Roots {
		dest := filepath.Join(outDir, filepath.FromSlash(strings.TrimPrefix(root, "/")))
		if err := os.MkdirAll(filepath.Dir(dest), 0o700); err != nil {
			return nil, err
		}
		if _, err := r.Run("adb", "-s", serial, "pull", root, dest); err != nil {
			m.Skipped = append(m.Skipped, root)
		}
	}
	files, err := hashTree(outDir)
	if err != nil {
		return nil, err
	}
	m.Files = files
	if len(m.Files) == 0 {
		return m, errors.New("nenhum arquivo foi copiado: o aparelho não deixou ler /vendor sem root")
	}
	b, err := json.MarshalIndent(m, "", "  ")
	if err != nil {
		return nil, err
	}
	if err := os.WriteFile(filepath.Join(outDir, "extract-manifest.json"), b, 0o600); err != nil {
		return nil, err
	}
	return m, nil
}

func prop(r device.Runner, serial, key string) string {
	out, err := r.Run("adb", "-s", serial, "shell", "getprop", key)
	if err != nil {
		return ""
	}
	return strings.TrimSpace(out)
}

// hashTree lista e calcula o SHA-256 de cada arquivo comum copiado (ignora links e os arquivos de controle).
func hashTree(dir string) ([]File, error) {
	var files []File
	err := filepath.WalkDir(dir, func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		rel, _ := filepath.Rel(dir, p)
		if d.IsDir() || rel == ".gitignore" || rel == "AVISO.txt" || rel == "extract-manifest.json" || !d.Type().IsRegular() {
			return nil
		}
		f, err := os.Open(p)
		if err != nil {
			return err
		}
		defer f.Close()
		h := sha256.New()
		n, err := io.Copy(h, f)
		if err != nil {
			return err
		}
		files = append(files, File{Path: filepath.ToSlash(rel), Size: n, SHA256: hex.EncodeToString(h.Sum(nil))})
		return nil
	})
	sort.Slice(files, func(i, j int) bool { return files[i].Path < files[j].Path })
	return files, err
}
