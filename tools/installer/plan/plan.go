// Package plan monta o plano de instalação: o core do sistema é obrigatório; GApps, APKs e
// pacotes de driver são extras que a pessoa escolhe, validados antes de qualquer comando.
package plan

import (
	"archive/zip"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/safety"
)

// Ordem de gravação do core. boot, vbmeta e super são obrigatórios (safety.RequiredImages).
var CoreImageOrder = []string{"boot", "dtbo", "vbmeta", "vendor_boot", "super", "recovery"}

// Kind classifica um extra escolhido.
type Kind string

const (
	KindGApps  Kind = "gapps"
	KindAPK    Kind = "apk"
	KindDriver Kind = "driver"
)

// Extra é um arquivo escolhido pela pessoa. O instalador não baixa nem redistribui nenhum deles.
type Extra struct {
	Kind   Kind
	Path   string
	SHA256 string
	Size   int64
}

// Options são as escolhas da pessoa.
type Options struct {
	GApps               string   // zip de GApps fornecido pela pessoa (opcional)
	APKs                []string // APKs comuns
	DriverBundles       []string // APKs de bundle de driver (ADR-014)
	ExperimentalDrivers bool     // permite bundle de marca ainda não verificada
}

// Core é a release do sistema já baixada, verificada e extraída.
type Core struct {
	Version string
	// Images mapeia partição → caminho do .img extraído.
	Images map[string]string
}

// Step é uma ação do plano. Exatamente um entre Cmd, Wait, Check e Manual está preenchido.
type Step struct {
	Phase  string
	Title  string
	Cmd    []string // adb ou fastboot, argv completo
	Wait   string   // "fastboot" ou "boot"
	Check  string   // "product"
	Manual string   // instrução para a pessoa fazer no aparelho
}

// Plan é o que será feito, na ordem.
type Plan struct {
	Device   device.Device
	Core     Core
	Extras   []Extra
	Steps    []Step
	Warnings []string
}

// BrandInfo vem dos descritores em brands/*.json.
type BrandInfo struct {
	Brand   string `json:"brand"`
	Status  string `json:"status"`
	Devices []struct {
		Codename string `json:"codename"`
	} `json:"devices"`
}

// codenameBrand liga o codinome ao descritor de marca (aparelhos hoje suportados pelo instalador).
var codenameBrand = map[string]string{"akita": "google", "pong": "nothing"}

// LoadBrands lê os descritores de marca de um diretório.
func LoadBrands(dir string) ([]BrandInfo, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var out []BrandInfo
	for _, e := range entries {
		if e.IsDir() || !strings.HasSuffix(e.Name(), ".json") {
			continue
		}
		b, err := os.ReadFile(filepath.Join(dir, e.Name()))
		if err != nil {
			return nil, err
		}
		var bi BrandInfo
		if err := json.Unmarshal(b, &bi); err != nil {
			return nil, fmt.Errorf("%s: %w", e.Name(), err)
		}
		out = append(out, bi)
	}
	return out, nil
}

// DriverStatus devolve "verified" só se um descritor verified listar o codinome; "unverified" se a
// marca do aparelho existe mas sem evidência; "unknown" nos demais casos.
func DriverStatus(brands []BrandInfo, codename string) string {
	want := codenameBrand[codename]
	for _, b := range brands {
		if b.Status == "verified" {
			for _, d := range b.Devices {
				if d.Codename == codename {
					return "verified"
				}
			}
		}
	}
	for _, b := range brands {
		if b.Brand == want && want != "" {
			return "unverified"
		}
	}
	return "unknown"
}

// ErrInvalid cobre qualquer escolha recusada.
var ErrInvalid = errors.New("escolha recusada")

func invalid(format string, a ...any) error {
	return fmt.Errorf("%w: %s", ErrInvalid, fmt.Sprintf(format, a...))
}

const maxExtraSize = int64(4) << 30

func inspect(kind Kind, path string) (Extra, error) {
	st, err := os.Stat(path)
	if err != nil {
		return Extra{}, invalid("%s: %v", path, err)
	}
	if !st.Mode().IsRegular() {
		return Extra{}, invalid("%s não é um arquivo comum", path)
	}
	if st.Size() == 0 || st.Size() > maxExtraSize {
		return Extra{}, invalid("%s: tamanho inválido (%d bytes)", path, st.Size())
	}
	zr, err := zip.OpenReader(path)
	if err != nil {
		return Extra{}, invalid("%s não é um pacote zip/apk válido: %v", path, err)
	}
	defer zr.Close()
	switch kind {
	case KindGApps:
		if !strings.EqualFold(filepath.Ext(path), ".zip") {
			return Extra{}, invalid("GApps precisa ser um .zip: %s", path)
		}
	case KindAPK, KindDriver:
		if !strings.EqualFold(filepath.Ext(path), ".apk") {
			return Extra{}, invalid("esperava um .apk: %s", path)
		}
		hasManifest := false
		for _, f := range zr.File {
			if f.Name == "AndroidManifest.xml" {
				hasManifest = true
			}
		}
		if !hasManifest {
			return Extra{}, invalid("%s não tem AndroidManifest.xml", path)
		}
	}
	f, err := os.Open(path)
	if err != nil {
		return Extra{}, err
	}
	defer f.Close()
	h := sha256.New()
	if _, err := io.Copy(h, f); err != nil {
		return Extra{}, err
	}
	return Extra{Kind: kind, Path: path, SHA256: hex.EncodeToString(h.Sum(nil)), Size: st.Size()}, nil
}

// Build valida tudo e devolve o plano. Nenhum comando é executado aqui.
func Build(dev device.Device, core Core, opts Options, brands []BrandInfo) (*Plan, error) {
	if dev.Serial == "" || dev.Codename == "" {
		return nil, invalid("aparelho não identificado")
	}
	for _, req := range []string{"boot", "vbmeta", "super"} {
		if core.Images[req] == "" {
			return nil, invalid("imagem obrigatória ausente no core: %s.img", req)
		}
	}
	p := &Plan{Device: dev, Core: core}

	seen := map[string]bool{}
	add := func(kind Kind, path string) error {
		e, err := inspect(kind, path)
		if err != nil {
			return err
		}
		if seen[e.SHA256] {
			return invalid("arquivo repetido: %s", path)
		}
		seen[e.SHA256] = true
		p.Extras = append(p.Extras, e)
		return nil
	}
	if opts.GApps != "" {
		if err := add(KindGApps, opts.GApps); err != nil {
			return nil, err
		}
		p.Warnings = append(p.Warnings,
			"GApps é fornecido por você: o Conexus não o redistribui. Ele precisa ser da mesma versão do Android da imagem (Android 16). Instalar GApps depois do primeiro boot exige apagar os dados.")
	}
	for _, a := range opts.APKs {
		if err := add(KindAPK, a); err != nil {
			return nil, err
		}
	}
	if len(opts.DriverBundles) > 0 {
		switch st := DriverStatus(brands, dev.Codename); {
		case st == "verified":
		case opts.ExperimentalDrivers:
			p.Warnings = append(p.Warnings, fmt.Sprintf(
				"Pacote de driver em modo EXPERIMENTAL: a marca de %s está '%s', sem evidência C1–C5. O conexus-driver-manager ainda recusa pacote não assinado.", dev.Codename, st))
		default:
			return nil, invalid("nenhum driver verificado para %q (estado: %s); use --experimental-drivers para instalar por sua conta e risco", dev.Codename, st)
		}
		for _, d := range opts.DriverBundles {
			if err := add(KindDriver, d); err != nil {
				return nil, err
			}
		}
	}
	if dev.BootloaderUnlocked != nil && !*dev.BootloaderUnlocked {
		return nil, invalid("o bootloader está travado; destrave-o antes (apaga todos os dados)")
	}
	if dev.BootloaderUnlocked == nil {
		p.Warnings = append(p.Warnings, "Não consegui confirmar se o bootloader está destravado.")
	}
	p.Warnings = append(p.Warnings, "A instalação apaga todos os dados do aparelho.")

	s, serial := &p.Steps, dev.Serial
	// 1. Core
	if dev.Mode == device.ModeADB {
		*s = append(*s,
			Step{Phase: "core", Title: "Reiniciar no bootloader", Cmd: []string{"adb", "-s", serial, "reboot", "bootloader"}},
			Step{Phase: "core", Title: "Aguardar o fastboot", Wait: "fastboot"})
	}
	*s = append(*s, Step{Phase: "core", Title: "Conferir que o aparelho é o certo", Check: "product"})
	for _, part := range CoreImageOrder {
		if img := core.Images[part]; img != "" {
			*s = append(*s, Step{Phase: "core", Title: "Gravar " + part, Cmd: []string{"fastboot", "-s", serial, "flash", part, img}})
		}
	}
	// 2. GApps, antes do primeiro boot
	var gapps *Extra
	for i := range p.Extras {
		if p.Extras[i].Kind == KindGApps {
			gapps = &p.Extras[i]
		}
	}
	if gapps != nil {
		*s = append(*s,
			Step{Phase: "gapps", Title: "Reiniciar no recovery", Cmd: []string{"fastboot", "-s", serial, "reboot", "recovery"}},
			Step{Phase: "gapps", Title: "No aparelho", Manual: "No recovery, escolha: Apply update → Apply from ADB. Depois pressione Enter aqui."},
			Step{Phase: "gapps", Title: "Instalar o GApps escolhido", Cmd: []string{"adb", "-s", serial, "sideload", gapps.Path}})
	}
	// 3. Primeiro boot
	if gapps != nil {
		*s = append(*s, Step{Phase: "boot", Title: "Reiniciar no sistema", Cmd: []string{"adb", "-s", serial, "reboot"}})
	} else {
		*s = append(*s, Step{Phase: "boot", Title: "Reiniciar no sistema", Cmd: []string{"fastboot", "-s", serial, "reboot"}})
	}
	*s = append(*s, Step{Phase: "boot", Title: "Aguardar o sistema iniciar", Wait: "boot"})
	// 4. APKs, drivers por último
	for _, kind := range []Kind{KindAPK, KindDriver} {
		for _, e := range p.Extras {
			if e.Kind == kind {
				*s = append(*s, Step{Phase: "apps", Title: "Instalar " + filepath.Base(e.Path), Cmd: []string{"adb", "-s", serial, "install", "-r", e.Path}})
			}
		}
	}
	return p, nil
}

// Confirmation é a frase que a pessoa digita para autorizar o plano.
func (p *Plan) Confirmation() string {
	return safety.ConfirmationPhrase(p.Device.Serial, p.Device.Codename)
}
