// Package apply executa um plano já validado. Fecha em caso de dúvida: a primeira falha para tudo,
// e nada é executado antes de a pessoa digitar a frase de confirmação deste aparelho.
package apply

import (
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/plan"
)

// Options liga o executor ao terminal e ao aparelho.
type Options struct {
	Runner device.Runner
	// Prompt mostra a mensagem e devolve o que a pessoa digitou.
	Prompt func(message string) string
	// Pause mostra a mensagem e espera a pessoa continuar (passos manuais no aparelho).
	Pause func(message string)
	Log   func(format string, a ...any)
	Sleep func(time.Duration)
	// PollAttempts e PollEvery controlam as esperas; padrão: 120 tentativas a cada 2s.
	PollAttempts int
	PollEvery    time.Duration
	DryRun       bool
}

// ErrNotConfirmed indica que a frase digitada não confere; nada foi executado.
var ErrNotConfirmed = errors.New("confirmação não confere; nada foi alterado")

// Execute roda o plano.
func Execute(p *plan.Plan, o Options) error {
	if o.Log == nil {
		o.Log = func(string, ...any) {}
	}
	if o.Sleep == nil {
		o.Sleep = time.Sleep
	}
	if o.PollAttempts == 0 {
		o.PollAttempts = 120
	}
	if o.PollEvery == 0 {
		o.PollEvery = 2 * time.Second
	}
	if o.DryRun {
		for _, s := range p.Steps {
			o.Log("[simulação] %s: %s %s", s.Phase, s.Title, strings.Join(s.Cmd, " "))
		}
		return nil
	}
	if o.Runner == nil || o.Prompt == nil || o.Pause == nil {
		return errors.New("executor mal configurado")
	}
	phrase := p.Confirmation()
	if strings.TrimSpace(o.Prompt("Para autorizar, digite exatamente: "+phrase)) != phrase {
		return ErrNotConfirmed
	}
	for i, s := range p.Steps {
		o.Log("(%d/%d) %s", i+1, len(p.Steps), s.Title)
		if err := runStep(p, s, o); err != nil {
			return fmt.Errorf("passo %q falhou: %w", s.Title, err)
		}
	}
	return nil
}

func runStep(p *plan.Plan, s plan.Step, o Options) error {
	serial := p.Device.Serial
	switch {
	case len(s.Cmd) > 0:
		out, err := o.Runner.Run(s.Cmd[0], s.Cmd[1:]...)
		if err != nil {
			return fmt.Errorf("%w: %s", err, strings.TrimSpace(out))
		}
	case s.Manual != "":
		o.Pause(s.Manual)
	case s.Check == "product":
		out, err := o.Runner.Run("fastboot", "-s", serial, "getvar", "product")
		if err != nil {
			return err
		}
		if got := fastbootVar(out, "product"); got != p.Device.Codename {
			return fmt.Errorf("o aparelho reporta %q, esperado %q; nada foi gravado", got, p.Device.Codename)
		}
	case s.Wait == "fastboot":
		return poll(o, func() bool {
			out, _ := o.Runner.Run("fastboot", "devices")
			return strings.Contains(out, serial+"\tfastboot") || strings.Contains(out, serial+" fastboot")
		}, "o aparelho não apareceu em modo fastboot")
	case s.Wait == "boot":
		return poll(o, func() bool {
			out, _ := o.Runner.Run("adb", "-s", serial, "shell", "getprop", "sys.boot_completed")
			return strings.TrimSpace(out) == "1"
		}, "o sistema não terminou de iniciar a tempo")
	default:
		return errors.New("passo vazio")
	}
	return nil
}

func poll(o Options, ok func() bool, msg string) error {
	for i := 0; i < o.PollAttempts; i++ {
		if ok() {
			return nil
		}
		o.Sleep(o.PollEvery)
	}
	return errors.New(msg)
}

func fastbootVar(out, key string) string {
	for _, line := range strings.Split(out, "\n") {
		line = strings.TrimSpace(line)
		if strings.HasPrefix(strings.ToLower(line), key+":") {
			return strings.TrimSpace(line[len(key)+1:])
		}
	}
	return ""
}
