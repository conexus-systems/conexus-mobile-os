package extract

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// fake simula o aparelho: "pull" cria arquivos no destino.
type fake struct {
	files map[string]map[string]string // raiz → {nome: conteúdo}
	deny  map[string]bool
	calls []string
}

func (f *fake) Run(name string, args ...string) (string, error) {
	line := name + " " + strings.Join(args, " ")
	f.calls = append(f.calls, line)
	if len(args) >= 5 && args[2] == "pull" {
		root, dest := args[3], args[4]
		if f.deny[root] {
			return "", errors.New("permission denied")
		}
		for n, c := range f.files[root] {
			p := filepath.Join(dest, n)
			os.MkdirAll(filepath.Dir(p), 0o700)
			os.WriteFile(p, []byte(c), 0o600)
		}
		return "", nil
	}
	switch {
	case strings.HasSuffix(line, "ro.product.model"):
		return "TCL 505\n", nil
	case strings.HasSuffix(line, "ro.product.device"):
		return "dev\n", nil
	}
	return "", nil
}

func TestRunCopiesAllowedRootsAndHashes(t *testing.T) {
	f := &fake{files: map[string]map[string]string{
		"/vendor/lib/modules": {"touch.ko": "KO", "wifi.ko": "WIFI"},
		"/vendor/firmware":    {"fw.bin": "FW"},
	}}
	out := filepath.Join(t.TempDir(), "out")
	m, err := Run(f, "S1", out)
	if err != nil {
		t.Fatal(err)
	}
	if len(m.Files) != 3 || m.Model != "TCL 505" {
		t.Fatalf("manifesto inesperado: %+v", m)
	}
	sum := sha256.Sum256([]byte("KO"))
	found := false
	for _, x := range m.Files {
		if strings.HasSuffix(x.Path, "touch.ko") && x.SHA256 == hex.EncodeToString(sum[:]) {
			found = true
		}
	}
	if !found {
		t.Error("hash de touch.ko não confere")
	}
	gi, _ := os.ReadFile(filepath.Join(out, ".gitignore"))
	if strings.TrimSpace(string(gi)) != "*" {
		t.Error(".gitignore deveria ignorar tudo")
	}
	if _, err := os.Stat(filepath.Join(out, "AVISO.txt")); err != nil {
		t.Error("falta o aviso de não redistribuir")
	}
	b, _ := os.ReadFile(filepath.Join(out, "extract-manifest.json"))
	var back Manifest
	if json.Unmarshal(b, &back) != nil || len(back.Files) != 3 {
		t.Error("manifesto gravado inválido")
	}
	if strings.Contains(string(b), "S1") {
		t.Error("o serial não pode aparecer no manifesto")
	}
}

func TestRunOnlyPullsAllowedRoots(t *testing.T) {
	f := &fake{files: map[string]map[string]string{"/vendor/lib/modules": {"a.ko": "x"}}}
	if _, err := Run(f, "S1", filepath.Join(t.TempDir(), "o")); err != nil {
		t.Fatal(err)
	}
	for _, c := range f.calls {
		if !strings.Contains(c, " pull ") {
			continue
		}
		ok := false
		for _, r := range Roots {
			if strings.Contains(c, " pull "+r+" ") {
				ok = true
			}
		}
		if !ok {
			t.Errorf("pull fora da lista permitida: %s", c)
		}
		if strings.Contains(c, "/data") || strings.Contains(c, "/sdcard") {
			t.Errorf("nunca deve ler dados do usuário: %s", c)
		}
	}
}

func TestRunRecordsDeniedRootsAndContinues(t *testing.T) {
	f := &fake{
		files: map[string]map[string]string{"/vendor/firmware": {"fw.bin": "FW"}},
		deny:  map[string]bool{"/vendor/lib/modules": true},
	}
	m, err := Run(f, "S1", filepath.Join(t.TempDir(), "o"))
	if err != nil || len(m.Files) != 1 {
		t.Fatalf("deveria seguir com o que deu: %+v %v", m, err)
	}
	if len(m.Skipped) == 0 || m.Skipped[0] != "/vendor/lib/modules" && !contains(m.Skipped, "/vendor/lib/modules") {
		t.Errorf("a raiz negada deveria constar em skipped: %v", m.Skipped)
	}
}

func contains(l []string, s string) bool {
	for _, x := range l {
		if x == s {
			return true
		}
	}
	return false
}

func TestRunFailsWhenNothingCopiedOrDirNotEmpty(t *testing.T) {
	if _, err := Run(&fake{deny: map[string]bool{}}, "S1", filepath.Join(t.TempDir(), "o")); err == nil {
		t.Error("sem nenhum arquivo copiado deveria falhar")
	}
	dir := t.TempDir()
	os.WriteFile(filepath.Join(dir, "x"), []byte("x"), 0o600)
	if _, err := Run(&fake{}, "S1", dir); err == nil {
		t.Error("pasta não vazia deveria ser recusada")
	}
}
