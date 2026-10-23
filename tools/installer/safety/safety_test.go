package safety

import (
	"archive/zip"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

const h = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

func TestIsValidSHA256(t *testing.T) {
	if !IsValidSHA256(h) {
		t.Fatal("hash válido recusado")
	}
	for _, bad := range []string{"", "abc", strings.Repeat("g", 64), h + "0"} {
		if IsValidSHA256(bad) {
			t.Errorf("hash inválido aceito: %q", bad)
		}
	}
}

func TestParseSHA256Sums(t *testing.T) {
	text := h + "  conexus-akita.zip\n" + strings.Repeat("B", 64) + " *other.zip\nlixo\n" + strings.Repeat("c", 10) + "  curto.zip\n"
	got := ParseSHA256Sums(text)
	if len(got) != 2 || got["conexus-akita.zip"] != h || got["other.zip"] != strings.Repeat("b", 64) {
		t.Fatalf("resultado inesperado: %v", got)
	}
}

func TestVerifySHA256(t *testing.T) {
	p := filepath.Join(t.TempDir(), "f.bin")
	if err := os.WriteFile(p, []byte("conteudo"), 0o600); err != nil {
		t.Fatal(err)
	}
	sum := sha256.Sum256([]byte("conteudo"))
	good := hex.EncodeToString(sum[:])
	if err := VerifySHA256(p, good); err != nil {
		t.Errorf("hash correto recusado: %v", err)
	}
	if err := VerifySHA256(p, strings.ToUpper(good)); err != nil {
		t.Errorf("hash em maiúsculas recusado: %v", err)
	}
	for _, bad := range []string{h, "", "a1b2c3"} {
		if err := VerifySHA256(p, bad); !errors.Is(err, ErrUnsafe) {
			t.Errorf("hash %q deveria ser recusado, obtido %v", bad, err)
		}
	}
}

func makeZip(t *testing.T, entries map[string]string, symlink string) string {
	t.Helper()
	p := filepath.Join(t.TempDir(), "p.zip")
	f, err := os.Create(p)
	if err != nil {
		t.Fatal(err)
	}
	zw := zip.NewWriter(f)
	for name, data := range entries {
		w, err := zw.Create(name)
		if err != nil {
			t.Fatal(err)
		}
		w.Write([]byte(data))
	}
	if symlink != "" {
		hdr := &zip.FileHeader{Name: symlink}
		hdr.SetMode(os.ModeSymlink | 0o777)
		w, err := zw.CreateHeader(hdr)
		if err != nil {
			t.Fatal(err)
		}
		w.Write([]byte("/etc/passwd"))
	}
	zw.Close()
	f.Close()
	return p
}

func TestSafeExtractFlat(t *testing.T) {
	z := makeZip(t, map[string]string{"boot.img": "1", "super.img": "2", "notes.txt": "n"}, "")
	out, err := SafeExtract(z, filepath.Join(t.TempDir(), "out"), DefaultMaxTotal)
	if err != nil || len(out) != 3 {
		t.Fatalf("esperava 3 arquivos, obtido %v, %v", out, err)
	}
}

func TestSafeExtractRejects(t *testing.T) {
	cases := map[string]map[string]string{
		"traversal":   {"../evil.img": "x"},
		"absoluto":    {"/abs.img": "x"},
		"subpasta":    {"dir/boot.img": "x"},
		"contrabarra": {`a\b.img`: "x"},
		"tipo":        {"run.sh": "#!/bin/sh"},
	}
	for name, entries := range cases {
		z := makeZip(t, entries, "")
		if _, err := SafeExtract(z, filepath.Join(t.TempDir(), "out"), DefaultMaxTotal); !errors.Is(err, ErrUnsafe) {
			t.Errorf("%s: deveria ser recusado, obtido %v", name, err)
		}
	}
}

func TestSafeExtractRejectsSymlinkEntriesAndSize(t *testing.T) {
	z := makeZip(t, map[string]string{"boot.img": "1"}, "link.img")
	if _, err := SafeExtract(z, filepath.Join(t.TempDir(), "out"), DefaultMaxTotal); !errors.Is(err, ErrUnsafe) {
		t.Errorf("link simbólico deveria ser recusado, obtido %v", err)
	}
	many := map[string]string{}
	for i := 0; i < 65; i++ {
		many[strings.Repeat("a", i+1)+".img"] = "x"
	}
	if _, err := SafeExtract(makeZip(t, many, ""), filepath.Join(t.TempDir(), "o"), DefaultMaxTotal); !errors.Is(err, ErrUnsafe) {
		t.Errorf("65 entradas deveria ser recusado, obtido %v", err)
	}
	if _, err := SafeExtract(makeZip(t, map[string]string{"boot.img": strings.Repeat("x", 100)}, ""), filepath.Join(t.TempDir(), "o"), 10); !errors.Is(err, ErrUnsafe) {
		t.Errorf("tamanho acima do limite deveria ser recusado, obtido %v", err)
	}
}

func TestRequireImages(t *testing.T) {
	if err := RequireImages([]string{"x/boot.img", "x/vbmeta.img", "x/super.img"}); err != nil {
		t.Errorf("imagens completas recusadas: %v", err)
	}
	if err := RequireImages([]string{"boot.img", "super.img"}); !errors.Is(err, ErrUnsafe) {
		t.Errorf("sem vbmeta deveria ser recusado, obtido %v", err)
	}
}

func TestConfirmationPhrase(t *testing.T) {
	if got := ConfirmationPhrase("ABC123", "akita"); got != "FLASH akita ABC123" {
		t.Errorf("frase inesperada: %q", got)
	}
	if ConfirmationPhrase("ABC123", "akita") == ConfirmationPhrase("XYZ", "akita") {
		t.Error("a frase precisa depender do serial")
	}
}
