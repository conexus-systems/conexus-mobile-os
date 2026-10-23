package apply

import (
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/plan"
)

type rec struct {
	calls   []string
	product string
	failOn  string
	booted  bool
}

func (r *rec) Run(name string, args ...string) (string, error) {
	line := name + " " + strings.Join(args, " ")
	r.calls = append(r.calls, line)
	switch {
	case r.failOn != "" && strings.Contains(line, r.failOn):
		return "erro simulado", errors.New("falhou")
	case strings.Contains(line, "getvar product"):
		return "product: " + r.product + "\n", nil
	case strings.HasPrefix(line, "fastboot devices"):
		return "S1\tfastboot\n", nil
	case strings.Contains(line, "sys.boot_completed"):
		return "1\n", nil
	}
	return "", nil
}

func buildPlan(t *testing.T) *plan.Plan {
	t.Helper()
	imgs := map[string]string{}
	for _, n := range plan.CoreImageOrder {
		p := filepath.Join(t.TempDir(), n+".img")
		os.WriteFile(p, []byte(n), 0o600)
		imgs[n] = p
	}
	unlocked := true
	p, err := plan.Build(device.Device{Serial: "S1", Codename: "akita", Mode: device.ModeFastboot, BootloaderUnlocked: &unlocked},
		plan.Core{Version: "v1", Images: imgs}, plan.Options{}, nil)
	if err != nil {
		t.Fatal(err)
	}
	return p
}

func opts(r *rec, typed string) Options {
	return Options{
		Runner: r,
		Prompt: func(string) string { return typed },
		Pause:  func(string) {},
		Sleep:  func(time.Duration) {},
	}
}

func flashed(r *rec) int {
	n := 0
	for _, c := range r.calls {
		if strings.Contains(c, " flash ") {
			n++
		}
	}
	return n
}

func TestWrongConfirmationRunsNothing(t *testing.T) {
	r := &rec{product: "akita"}
	err := Execute(buildPlan(t), opts(r, "sim"))
	if !errors.Is(err, ErrNotConfirmed) || len(r.calls) != 0 {
		t.Fatalf("esperava recusa sem comandos, obtido %v e %v", err, r.calls)
	}
}

func TestCorrectConfirmationFlashesInOrder(t *testing.T) {
	r := &rec{product: "akita"}
	p := buildPlan(t)
	if err := Execute(p, opts(r, "FLASH akita S1")); err != nil {
		t.Fatal(err)
	}
	if flashed(r) != 6 {
		t.Errorf("esperava 6 partições gravadas, obtido %d: %v", flashed(r), r.calls)
	}
	last := r.calls[len(r.calls)-1]
	if !strings.Contains(last, "sys.boot_completed") {
		t.Errorf("deveria terminar esperando o boot: %s", last)
	}
}

func TestWrongProductAbortsBeforeFlashing(t *testing.T) {
	r := &rec{product: "outro"}
	err := Execute(buildPlan(t), opts(r, "FLASH akita S1"))
	if err == nil || flashed(r) != 0 {
		t.Fatalf("deveria abortar antes de gravar: %v, gravou %d", err, flashed(r))
	}
}

func TestFirstFailureStopsEverything(t *testing.T) {
	r := &rec{product: "akita", failOn: "flash vbmeta"}
	err := Execute(buildPlan(t), opts(r, "FLASH akita S1"))
	if err == nil {
		t.Fatal("deveria falhar")
	}
	for _, c := range r.calls {
		if strings.Contains(c, "flash super") || strings.HasSuffix(c, " reboot") {
			t.Errorf("nada deveria rodar depois da falha: %s", c)
		}
	}
}

func TestDryRunExecutesNothing(t *testing.T) {
	r := &rec{product: "akita"}
	o := opts(r, "")
	o.DryRun = true
	if err := Execute(buildPlan(t), o); err != nil || len(r.calls) != 0 {
		t.Fatalf("simulação não pode executar comandos: %v %v", err, r.calls)
	}
}

func TestBootTimeoutIsAnError(t *testing.T) {
	r := &neverBoots{rec{product: "akita"}}
	o := opts(&r.rec, "FLASH akita S1")
	o.Runner = r
	o.PollAttempts = 3
	if err := Execute(buildPlan(t), o); err == nil || !strings.Contains(fmt.Sprint(err), "iniciar") {
		t.Fatalf("esperava erro de boot, obtido %v", err)
	}
}

type neverBoots struct{ rec }

func (n *neverBoots) Run(name string, args ...string) (string, error) {
	if strings.Contains(strings.Join(args, " "), "sys.boot_completed") {
		return "0\n", nil
	}
	return n.rec.Run(name, args...)
}
