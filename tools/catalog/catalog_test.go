package main

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func write(t *testing.T, dir, name, body string) {
	t.Helper()
	if err := os.WriteFile(filepath.Join(dir, name), []byte(body), 0o644); err != nil {
		t.Fatal(err)
	}
}

func fixture(t *testing.T) []Device {
	t.Helper()
	d := t.TempDir()
	write(t, d, "akita.yml", "codename: akita\nname: Pixel 8a\nvendor: Google\nvendor_short: google\nsoc: Google Tensor G3\ntree: android_device_google_akita\ninstall_method: fastboot_nexus\nis_ab_device: true\nversions: [21, 22.1, 23.0, 23.2, 24.0]\nkernel: {repo: android_kernel_google_gs-6.1_manifest, version: '6.1'}\n")
	write(t, d, "poco1.yml", "codename: alioth\nname: POCO F3\nvendor: Xiaomi\nvendor_short: xiaomi\nsoc: SD870\ntree: android_device_xiaomi_alioth\ninstall_method: fastboot_xiaomi\nversions: [23.2]\nkernel: {repo: android_kernel_xiaomi_sm8250, version: '4.19'}\n")
	write(t, d, "redmi1.yml", "codename: Mi8937\nname: Redmi 4\nvendor: Xiaomi\nvendor_short: xiaomi\nsoc: SD430\ntree: t\ninstall_method: fastboot_xiaomi\nversions: [23.2]\nkernel: {repo: k, version: '4.9'}\n")
	write(t, d, "old.yml", "codename: old\nname: Velho\nvendor: Google\nvendor_short: google\nsoc: x\ntree: t\ninstall_method: m\nversions: [18.1]\nkernel: {repo: k, version: '3.10'}\n")
	write(t, d, "sony.yml", "codename: s\nname: Xperia\nvendor: Sony\nvendor_short: sony\nsoc: x\ntree: t\ninstall_method: m\nversions: [23.2]\nkernel: {repo: k, version: '5.4'}\n")
	write(t, d, "moto.yml", "codename: devon\nname: moto g32\nvendor: Motorola\nvendor_short: motorola\nsoc: SM\ntree: t\ninstall_method: fastboot_motorola\nversions: [23.2]\nkernel: {repo: k, version: unknown}\n")
	devs, err := Load(d)
	if err != nil {
		t.Fatal(err)
	}
	return devs
}

func TestGroupByBrandAndVersion(t *testing.T) {
	g, skipped := Group(fixture(t), "23.2")
	if len(g["google"]) != 1 || len(g["xiaomi"]) != 2 || len(g["motorola"]) != 1 {
		t.Errorf("agrupamento inesperado: %v", g)
	}
	if skipped != 1 {
		t.Errorf("Sony deveria ser contada como fora do escopo, obtido %d", skipped)
	}
	for _, d := range g["google"] {
		if d.Codename == "old" {
			t.Error("aparelho sem a versão 23.2 não pode entrar")
		}
	}
}

func TestVersionsKeepOriginalText(t *testing.T) {
	d := fixture(t)[0] // akita
	if !d.Supports("23.0") || !d.Supports("23.2") || d.Supports("23") {
		t.Errorf("versões lidas: %v", d.Versions)
	}
}

func TestSubBrandsAndKernelHint(t *testing.T) {
	g, _ := Group(fixture(t), "23.2")
	subs := map[string]string{}
	for _, d := range g["xiaomi"] {
		subs[d.Codename] = SubBrand("xiaomi", d)
	}
	if subs["alioth"] != "poco" || subs["Mi8937"] != "redmi" {
		t.Errorf("linhas: %v", subs)
	}
	if SubBrand("motorola", g["motorola"][0]) != "moto-g" {
		t.Error("moto g32 deveria ser moto-g")
	}
	for in, want := range map[string]string{"6.1": "sim", "5.10": "sim", "5.4": "não", "4.19": "não", "unknown": "desconhecido", "": "desconhecido", "6.6": "sim"} {
		var d Device
		d.Kernel.Version = in
		if got := d.KernelAtLeast510(); got != want {
			t.Errorf("kernel %q: %s, esperado %s", in, got, want)
		}
	}
}

func TestRenderIsDeterministicAndHonest(t *testing.T) {
	g, _ := Group(fixture(t), "23.2")
	m := Meta{Version: "23.2", WikiCommit: "abc123", Date: "2026-10-22"}
	a, err := Render("xiaomi", g["xiaomi"], m)
	if err != nil {
		t.Fatal(err)
	}
	b, _ := Render("xiaomi", g["xiaomi"], m)
	for k := range a {
		if a[k] != b[k] {
			t.Errorf("%s não é determinístico", k)
		}
	}
	var desc descriptor
	if err := json.Unmarshal([]byte(a["brand-module.json"]), &desc); err != nil {
		t.Fatal(err)
	}
	if desc.Status != "unverified" || len(desc.Devices) != 2 || strings.Join(desc.SubBrands, ",") != "poco,redmi" {
		t.Errorf("descritor: %+v", desc)
	}
	if strings.Contains(a["brand-module.json"], "\"verified\"") || strings.Contains(a["devices.json"], "\"criteria\"") {
		t.Error("o catálogo não pode afirmar verificação nem preencher evidência")
	}
	if !strings.Contains(a["devices.json"], "\"c5_recovery\": \"não testado\"") || !strings.Contains(a["devices.json"], "abc123") {
		t.Error("C5 deve sair 'não testado' e a origem deve citar o commit")
	}
	if !strings.Contains(a["README.md"], "Não contém drivers") || !strings.Contains(a["SOURCES.md"], "Nada deles foi copiado") {
		t.Error("README/SOURCES devem dizer que não há drivers nem código copiado")
	}
}

func TestPlannedBrandHasNoDevices(t *testing.T) {
	f, _ := Render("tcl", nil, Meta{Version: "23.2", WikiCommit: "x", Date: "2026-10-22"})
	var desc descriptor
	json.Unmarshal([]byte(f["brand-module.json"]), &desc)
	if desc.Status != "planned" || len(desc.Devices) != 0 {
		t.Errorf("marca sem dados deve ser planned e sem aparelhos: %+v", desc)
	}
	if !strings.Contains(f["CATALOG.md"], "Nenhum aparelho") {
		t.Error("catálogo vazio deveria dizer que não há aparelhos")
	}
}

func TestLoadRejectsBrokenOrIncompleteFiles(t *testing.T) {
	d := t.TempDir()
	write(t, d, "bad.yml", "codename: [unclosed")
	if _, err := Load(d); err == nil {
		t.Error("YAML quebrado deveria ser erro")
	}
	d2 := t.TempDir()
	write(t, d2, "x.yml", "name: sem codinome\nvendor: X\n")
	if _, err := Load(d2); err == nil {
		t.Error("aparelho sem codinome deveria ser erro")
	}
	if _, err := Load(t.TempDir()); err == nil {
		t.Error("pasta vazia deveria ser erro")
	}
}

func TestRenderRefusesTooManyDevices(t *testing.T) {
	devs := make([]Device, 257)
	if _, err := Render("xiaomi", devs, Meta{}); err == nil {
		t.Error("mais de 256 aparelhos deveria ser recusado")
	}
}

func TestSoCAcceptsTextOrListButNotMappings(t *testing.T) {
	d := t.TempDir()
	write(t, d, "dual.yml", "codename: z\nname: Dual\nvendor: ASUS\nvendor_short: asus\nsoc: [Intel Atom Z3560, Intel Atom Z3580]\n")
	devs, err := Load(d)
	if err != nil || devs[0].SoC != "Intel Atom Z3560 / Intel Atom Z3580" {
		t.Fatalf("lista de SoC: %v %v", devs, err)
	}
	d2 := t.TempDir()
	write(t, d2, "bad.yml", "codename: z\nname: N\nvendor: V\nsoc: {a: b}\n")
	if _, err := Load(d2); err == nil {
		t.Error("SoC em mapa deveria ser erro")
	}
}
