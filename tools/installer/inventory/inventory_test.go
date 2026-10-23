package inventory

import (
	"encoding/json"
	"strings"
	"testing"
)

const sample = `[ro.product.manufacturer]: [TCL]
[ro.product.model]: [TCL 505]
[ro.product.device]: [wonder]
[ro.board.platform]: [mt6765]
[ro.boot.flash.locked]: [1]
[ro.serialno]: [SEGREDO123]
[ro.boot.serialno]: [SEGREDO123]
[ro.ril.oem.imei]: [111111111111111]
[persist.sys.timezone]: [America/Sao_Paulo]
lixo sem formato
`

func TestParseGetpropKeepsOnlyAllowedKeys(t *testing.T) {
	p := ParseGetprop(sample)
	if p["ro.product.model"] != "TCL 505" || p["ro.board.platform"] != "mt6765" {
		t.Errorf("chaves esperadas ausentes: %v", p)
	}
	for k := range p {
		if strings.Contains(k, "serial") || strings.Contains(k, "imei") || strings.Contains(k, "timezone") {
			t.Errorf("chave sensível guardada: %s", k)
		}
	}
	rep := &Report{Props: p}
	b, _ := rep.JSON()
	if strings.Contains(string(b), "SEGREDO123") || strings.Contains(string(b), "1111111") {
		t.Error("serial ou IMEI vazou para o JSON")
	}
}

type fakeRunner map[string]string

func (f fakeRunner) Run(name string, args ...string) (string, error) {
	if v, ok := f[name+" "+strings.Join(args, " ")]; ok {
		return v, nil
	}
	return "", errTest
}

type testErr struct{}

func (testErr) Error() string { return "falhou" }

var errTest = testErr{}

func TestCollect(t *testing.T) {
	r := fakeRunner{
		"adb -s S shell getprop":                sample,
		"adb -s S shell uname -r":               "4.19.191-g1234\n",
		"adb -s S shell ls /vendor/lib/modules": "a.ko b.ko readme.txt\n",
	}
	rep, err := Collect(r, "S")
	if err != nil || rep.Kernel != "4.19.191-g1234" || len(rep.Modules) != 2 {
		t.Fatalf("inesperado: %+v %v", rep, err)
	}
}

func TestCollectToleratesNoVendorListing(t *testing.T) {
	r := fakeRunner{"adb -s S shell getprop": sample}
	rep, err := Collect(r, "S")
	if err != nil || rep.Kernel != "" || len(rep.Modules) != 0 {
		t.Fatalf("sem root deveria seguir: %+v %v", rep, err)
	}
	if _, err := Collect(fakeRunner{"adb -s S shell getprop": "nada conhecido"}, "S"); err == nil {
		t.Error("sem nenhuma propriedade conhecida deveria falhar")
	}
}

func state(fs []Finding, prefix string) State {
	for _, f := range fs {
		if strings.HasPrefix(f.Criterion, prefix) {
			return f.State
		}
	}
	return "?"
}

func TestAssess(t *testing.T) {
	legacy := (&Report{Props: map[string]string{"ro.boot.flash.locked": "1"}, Kernel: "4.19.191"}).Assess()
	if state(legacy, "C4") != No || state(legacy, "C1") != Unknown || state(legacy, "C2") != Unknown || state(legacy, "C3") != Unknown {
		t.Errorf("kernel 4.19 travado: %+v", legacy)
	}
	modern := (&Report{Props: map[string]string{"ro.boot.flash.locked": "0", "ro.boot.slot_suffix": "_a"}, Kernel: "6.1.75-android14"}).Assess()
	if state(modern, "C4") != Yes || state(modern, "C1") != Yes {
		t.Errorf("kernel 6.1 destravado: %+v", modern)
	}
	if state(modern, "C5") != Unknown {
		t.Error("C5 nunca é aprovado por coleta: exige teste de recovery")
	}
	none := (&Report{Props: map[string]string{}}).Assess()
	if state(none, "C4") != Unknown || state(none, "C1") != Unknown {
		t.Errorf("sem dado deveria ser desconhecido: %+v", none)
	}
}

func TestKernelVersion(t *testing.T) {
	for in, want := range map[string][2]int{"5.10.198-android12": {5, 10}, "4.19.191": {4, 19}, "6.1": {6, 1}} {
		a, b, ok := KernelVersion(in)
		if !ok || a != want[0] || b != want[1] {
			t.Errorf("%s -> %d.%d %v", in, a, b, ok)
		}
	}
	if _, _, ok := KernelVersion("abc"); ok {
		t.Error("versão inválida aceita")
	}
}

func TestJSONHasAssessment(t *testing.T) {
	b, err := (&Report{Props: map[string]string{"ro.product.model": "x"}}).JSON()
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]any
	if err := json.Unmarshal(b, &m); err != nil || m["assessment"] == nil {
		t.Errorf("JSON sem avaliação: %s", b)
	}
}
