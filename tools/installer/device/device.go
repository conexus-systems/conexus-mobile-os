// Package device detecta o aparelho conectado e executa adb/fastboot sem shell.
package device

import (
	"bytes"
	"errors"
	"fmt"
	"os/exec"
	"regexp"
	"strings"
	"time"
)

// Runner executa um comando e devolve a saída combinada. Existe para os testes trocarem adb/fastboot.
type Runner interface {
	Run(name string, args ...string) (string, error)
}

// ExecRunner executa o comando de verdade, com argv fixo (nunca por shell), com limite de tempo.
type ExecRunner struct{ Timeout time.Duration }

func (r ExecRunner) Run(name string, args ...string) (string, error) {
	if name != "adb" && name != "fastboot" {
		return "", fmt.Errorf("comando não permitido: %q", name)
	}
	timeout := r.Timeout
	if timeout == 0 {
		timeout = 10 * time.Minute
	}
	cmd := exec.Command(name, args...)
	var buf bytes.Buffer
	cmd.Stdout, cmd.Stderr = &buf, &buf
	if err := cmd.Start(); err != nil {
		return "", err
	}
	done := make(chan error, 1)
	go func() { done <- cmd.Wait() }()
	select {
	case err := <-done:
		return buf.String(), err
	case <-time.After(timeout):
		_ = cmd.Process.Kill()
		return buf.String(), fmt.Errorf("%s: tempo esgotado (%s)", name, timeout)
	}
}

// Mode é o estado em que o aparelho foi encontrado.
type Mode string

const (
	ModeADB      Mode = "adb"
	ModeFastboot Mode = "fastboot"
)

// Device descreve o aparelho detectado.
type Device struct {
	Serial   string
	Codename string
	Mode     Mode
	// BootloaderUnlocked é nil quando não foi possível saber.
	BootloaderUnlocked *bool
}

var serialRe = regexp.MustCompile(`^[A-Za-z0-9._:-]{1,64}$`)
var codenameRe = regexp.MustCompile(`^[a-z0-9][a-z0-9_-]{0,63}$`)

// ErrNoDevice e ErrMultipleDevices evitam gravar no aparelho errado.
var (
	ErrNoDevice        = errors.New("nenhum aparelho conectado")
	ErrMultipleDevices = errors.New("mais de um aparelho conectado; desconecte os outros")
)

// Detect procura exatamente um aparelho em modo adb ou fastboot.
func Detect(r Runner) (*Device, error) {
	adbOut, _ := r.Run("adb", "devices")
	fbOut, _ := r.Run("fastboot", "devices")
	var found []Device
	for _, line := range strings.Split(adbOut, "\n") {
		f := strings.Fields(line)
		if len(f) == 2 && f[1] == "device" && serialRe.MatchString(f[0]) {
			found = append(found, Device{Serial: f[0], Mode: ModeADB})
		}
	}
	for _, line := range strings.Split(fbOut, "\n") {
		f := strings.Fields(line)
		if len(f) == 2 && f[1] == "fastboot" && serialRe.MatchString(f[0]) {
			found = append(found, Device{Serial: f[0], Mode: ModeFastboot})
		}
	}
	switch len(found) {
	case 0:
		return nil, ErrNoDevice
	case 1:
	default:
		return nil, ErrMultipleDevices
	}
	d := found[0]
	var err error
	switch d.Mode {
	case ModeADB:
		out, e := r.Run("adb", "-s", d.Serial, "shell", "getprop", "ro.product.device")
		d.Codename, err = strings.TrimSpace(out), e
	case ModeFastboot:
		out, e := r.Run("fastboot", "-s", d.Serial, "getvar", "product")
		d.Codename, err = parseFastbootVar(out, "product"), e
	}
	if err != nil {
		return nil, fmt.Errorf("não consegui ler o codinome: %w", err)
	}
	if !codenameRe.MatchString(d.Codename) {
		return nil, fmt.Errorf("codinome inválido: %q", d.Codename)
	}
	if d.Mode == ModeFastboot {
		out, _ := r.Run("fastboot", "-s", d.Serial, "getvar", "unlocked")
		switch strings.ToLower(parseFastbootVar(out, "unlocked")) {
		case "yes":
			t := true
			d.BootloaderUnlocked = &t
		case "no":
			f := false
			d.BootloaderUnlocked = &f
		}
	}
	return &d, nil
}

// parseFastbootVar lê "chave: valor" da saída do fastboot (que escreve em stderr).
func parseFastbootVar(out, key string) string {
	for _, line := range strings.Split(out, "\n") {
		line = strings.TrimSpace(line)
		if strings.HasPrefix(strings.ToLower(line), key+":") {
			return strings.TrimSpace(line[len(key)+1:])
		}
	}
	return ""
}
