package device

import (
	"errors"
	"strings"
	"testing"
)

type fake struct{ out map[string]string }

func (f fake) Run(name string, args ...string) (string, error) {
	key := name + " " + strings.Join(args, " ")
	if v, ok := f.out[key]; ok {
		return v, nil
	}
	return "", nil
}

func TestDetectADB(t *testing.T) {
	r := fake{map[string]string{
		"adb devices": "List of devices attached\n1A2B3C\tdevice\n",
		"adb -s 1A2B3C shell getprop ro.product.device": "akita\n",
	}}
	d, err := Detect(r)
	if err != nil || d.Serial != "1A2B3C" || d.Codename != "akita" || d.Mode != ModeADB {
		t.Fatalf("inesperado: %+v %v", d, err)
	}
}

func TestDetectFastbootUnlocked(t *testing.T) {
	r := fake{map[string]string{
		"fastboot devices":                "XYZ\tfastboot\n",
		"fastboot -s XYZ getvar product":  "product: pong\nFinished.\n",
		"fastboot -s XYZ getvar unlocked": "unlocked: yes\n",
	}}
	d, err := Detect(r)
	if err != nil || d.Codename != "pong" || d.BootloaderUnlocked == nil || !*d.BootloaderUnlocked {
		t.Fatalf("inesperado: %+v %v", d, err)
	}
}

func TestDetectNoneAndMultiple(t *testing.T) {
	if _, err := Detect(fake{}); !errors.Is(err, ErrNoDevice) {
		t.Errorf("sem aparelho: %v", err)
	}
	r := fake{map[string]string{"adb devices": "List\nA\tdevice\nB\tdevice\n"}}
	if _, err := Detect(r); !errors.Is(err, ErrMultipleDevices) {
		t.Errorf("dois aparelhos: %v", err)
	}
}

func TestDetectIgnoresUnauthorizedAndRejectsBadCodename(t *testing.T) {
	if _, err := Detect(fake{map[string]string{"adb devices": "List\nA\tunauthorized\n"}}); !errors.Is(err, ErrNoDevice) {
		t.Errorf("unauthorized deve contar como sem aparelho: %v", err)
	}
	r := fake{map[string]string{
		"adb devices": "List\nA\tdevice\n",
		"adb -s A shell getprop ro.product.device": "ak ita; rm -rf /\n",
	}}
	if _, err := Detect(r); err == nil {
		t.Error("codinome com caracteres perigosos deveria ser recusado")
	}
}

func TestExecRunnerRefusesOtherCommands(t *testing.T) {
	if _, err := (ExecRunner{}).Run("sh", "-c", "echo oi"); err == nil {
		t.Error("ExecRunner só pode executar adb e fastboot")
	}
}
