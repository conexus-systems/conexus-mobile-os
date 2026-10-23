package plan

import (
	"archive/zip"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
)

func writeZip(t *testing.T, dir, name string, entries map[string]string) string {
	t.Helper()
	p := filepath.Join(dir, name)
	f, err := os.Create(p)
	if err != nil {
		t.Fatal(err)
	}
	zw := zip.NewWriter(f)
	for n, d := range entries {
		w, _ := zw.Create(n)
		w.Write([]byte(d))
	}
	zw.Close()
	f.Close()
	return p
}

func core(t *testing.T) Core {
	t.Helper()
	imgs := map[string]string{}
	for _, n := range CoreImageOrder {
		p := filepath.Join(t.TempDir(), n+".img")
		os.WriteFile(p, []byte(n), 0o600)
		imgs[n] = p
	}
	return Core{Version: "v1", Images: imgs}
}

func tf(b bool) *bool { return &b }

func dev() device.Device {
	return device.Device{Serial: "S1", Codename: "akita", Mode: device.ModeFastboot, BootloaderUnlocked: tf(true)}
}

var verified = []BrandInfo{{Brand: "google", Status: "verified", Devices: []struct {
	Codename string `json:"codename"`
}{{Codename: "akita"}}}}
var unverified = []BrandInfo{{Brand: "google", Status: "unverified"}}

func titles(p *Plan) []string {
	var out []string
	for _, s := range p.Steps {
		out = append(out, s.Phase+":"+s.Title)
	}
	return out
}

func TestCoreOnlyPlan(t *testing.T) {
	p, err := Build(dev(), core(t), Options{}, unverified)
	if err != nil {
		t.Fatal(err)
	}
	got := strings.Join(titles(p), "|")
	if !strings.Contains(got, "core:Gravar boot") || !strings.Contains(got, "boot:Aguardar o sistema iniciar") {
		t.Errorf("plano sem core/boot: %s", got)
	}
	if strings.Contains(got, "gapps:") || strings.Contains(got, "apps:") {
		t.Errorf("sem extras não pode haver gapps/apps: %s", got)
	}
	if len(p.Extras) != 0 {
		t.Error("não deveria haver extras")
	}
}

func TestADBModeRebootsToBootloaderFirst(t *testing.T) {
	d := dev()
	d.Mode = device.ModeADB
	d.BootloaderUnlocked = nil
	p, err := Build(d, core(t), Options{}, nil)
	if err != nil {
		t.Fatal(err)
	}
	if p.Steps[0].Cmd[3] != "reboot" || p.Steps[0].Cmd[4] != "bootloader" {
		t.Errorf("primeiro passo deveria reiniciar no bootloader: %v", p.Steps[0])
	}
}

func TestGAppsGoesBeforeFirstBootAndAPKsAfter(t *testing.T) {
	dir := t.TempDir()
	gapps := writeZip(t, dir, "gapps.zip", map[string]string{"META-INF/x": "x"})
	apk := writeZip(t, dir, "app.apk", map[string]string{"AndroidManifest.xml": "m"})
	p, err := Build(dev(), core(t), Options{GApps: gapps, APKs: []string{apk}}, unverified)
	if err != nil {
		t.Fatal(err)
	}
	idx := func(prefix string) int {
		for i, s := range titles(p) {
			if strings.HasPrefix(s, prefix) {
				return i
			}
		}
		return -1
	}
	if !(idx("core:Gravar super") < idx("gapps:Instalar o GApps") && idx("gapps:Instalar o GApps") < idx("boot:Reiniciar") && idx("boot:Aguardar") < idx("apps:Instalar app.apk")) {
		t.Errorf("ordem errada: %v", titles(p))
	}
	manual := false
	for _, s := range p.Steps {
		if s.Manual != "" {
			manual = true
		}
	}
	if !manual {
		t.Error("o sideload exige um passo manual no recovery")
	}
	if !strings.Contains(strings.Join(p.Warnings, " "), "não o redistribui") {
		t.Error("deveria avisar que o GApps não é redistribuído")
	}
}

func TestDriversInstalledLastAndOnlyIfVerified(t *testing.T) {
	dir := t.TempDir()
	apk := writeZip(t, dir, "app.apk", map[string]string{"AndroidManifest.xml": "a"})
	drv := writeZip(t, dir, "drv.apk", map[string]string{"AndroidManifest.xml": "b"})
	p, err := Build(dev(), core(t), Options{APKs: []string{apk}, DriverBundles: []string{drv}}, verified)
	if err != nil {
		t.Fatal(err)
	}
	ts := titles(p)
	if ts[len(ts)-1] != "apps:Instalar drv.apk" {
		t.Errorf("driver deveria ser o último: %v", ts)
	}
	if _, err := Build(dev(), core(t), Options{DriverBundles: []string{drv}}, unverified); !errors.Is(err, ErrInvalid) {
		t.Errorf("driver de marca não verificada deveria ser recusado: %v", err)
	}
	p, err = Build(dev(), core(t), Options{DriverBundles: []string{drv}, ExperimentalDrivers: true}, unverified)
	if err != nil || !strings.Contains(strings.Join(p.Warnings, " "), "EXPERIMENTAL") {
		t.Errorf("modo experimental deveria passar com aviso: %v", err)
	}
}

func TestExtrasValidation(t *testing.T) {
	dir := t.TempDir()
	notZip := filepath.Join(dir, "x.apk")
	os.WriteFile(notZip, []byte("não sou zip"), 0o600)
	noManifest := writeZip(t, dir, "y.apk", map[string]string{"classes.dex": "d"})
	wrongExt := writeZip(t, dir, "z.txt", map[string]string{"AndroidManifest.xml": "m"})
	empty := filepath.Join(dir, "e.apk")
	os.WriteFile(empty, nil, 0o600)
	for name, o := range map[string]Options{
		"não-zip":       {APKs: []string{notZip}},
		"sem manifest":  {APKs: []string{noManifest}},
		"extensão":      {APKs: []string{wrongExt}},
		"vazio":         {APKs: []string{empty}},
		"inexistente":   {APKs: []string{filepath.Join(dir, "nada.apk")}},
		"gapps não zip": {GApps: notZip},
	} {
		if _, err := Build(dev(), core(t), o, nil); !errors.Is(err, ErrInvalid) {
			t.Errorf("%s: deveria ser recusado, obtido %v", name, err)
		}
	}
}

func TestDuplicateExtrasRefused(t *testing.T) {
	dir := t.TempDir()
	a := writeZip(t, dir, "a.apk", map[string]string{"AndroidManifest.xml": "m"})
	b := filepath.Join(dir, "b.apk")
	data, _ := os.ReadFile(a)
	os.WriteFile(b, data, 0o600)
	if _, err := Build(dev(), core(t), Options{APKs: []string{a, b}}, nil); !errors.Is(err, ErrInvalid) {
		t.Errorf("arquivos idênticos deveriam ser recusados: %v", err)
	}
}

func TestLockedBootloaderAndMissingImagesRefused(t *testing.T) {
	d := dev()
	d.BootloaderUnlocked = tf(false)
	if _, err := Build(d, core(t), Options{}, nil); !errors.Is(err, ErrInvalid) {
		t.Errorf("bootloader travado deveria ser recusado: %v", err)
	}
	c := core(t)
	delete(c.Images, "vbmeta")
	if _, err := Build(dev(), c, Options{}, nil); !errors.Is(err, ErrInvalid) {
		t.Errorf("sem vbmeta deveria ser recusado: %v", err)
	}
}

func TestDriverStatus(t *testing.T) {
	if DriverStatus(verified, "akita") != "verified" || DriverStatus(unverified, "akita") != "unverified" || DriverStatus(nil, "xyz") != "unknown" {
		t.Error("DriverStatus inesperado")
	}
}

func TestLoadBrandsFromRepo(t *testing.T) {
	brands, err := LoadBrands("../../../brands")
	if err != nil || len(brands) < 7 {
		t.Fatalf("brands/: %v (%d)", err, len(brands))
	}
	if DriverStatus(brands, "akita") == "verified" {
		t.Error("nenhuma marca está verified hoje")
	}
}
