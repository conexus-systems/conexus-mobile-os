// Package inventory coleta de um aparelho conectado os fatos de que o módulo de drivers depende
// (SoC, kernel, partições, bootloader) e os avalia contra os critérios C1–C5 do ADR-010.
// Só lê: não grava nada no aparelho e não copia arquivos de fabricante. Não coleta serial nem IMEI.
package inventory

import (
	"encoding/json"
	"fmt"
	"regexp"
	"sort"
	"strconv"
	"strings"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
)

// Chaves lidas de `getprop`. Lista fechada: nada fora dela é guardado (sem serial, IMEI, contas).
var propKeys = []string{
	"ro.product.manufacturer", "ro.product.brand", "ro.product.model", "ro.product.device", "ro.product.name",
	"ro.board.platform", "ro.hardware", "ro.soc.manufacturer", "ro.soc.model",
	"ro.product.cpu.abi", "ro.product.cpu.abilist",
	"ro.build.version.release", "ro.build.version.sdk", "ro.build.version.security_patch",
	"ro.product.first_api_level", "ro.build.fingerprint",
	"ro.boot.flash.locked", "ro.boot.verifiedbootstate", "ro.boot.vbmeta.device_state",
	"ro.boot.slot_suffix", "ro.boot.dynamic_partitions",
	"ro.treble.enabled", "ro.vndk.version", "ro.build.system_root_image", "ro.oem_unlock_supported",
}

// Report é o resultado da coleta.
type Report struct {
	Props   map[string]string `json:"props"`
	Kernel  string            `json:"kernel_release"`
	Modules []string          `json:"vendor_modules,omitempty"`
}

var propLine = regexp.MustCompile(`^\[([^\]]+)\]: \[(.*)\]$`)

// ParseGetprop lê a saída de `getprop` e devolve só as chaves da lista fechada.
func ParseGetprop(out string) map[string]string {
	allowed := map[string]bool{}
	for _, k := range propKeys {
		allowed[k] = true
	}
	props := map[string]string{}
	for _, line := range strings.Split(out, "\n") {
		if m := propLine.FindStringSubmatch(strings.TrimSpace(line)); m != nil && allowed[m[1]] {
			props[m[1]] = m[2]
		}
	}
	return props
}

// Collect lê o aparelho. Exige o modo adb.
func Collect(r device.Runner, serial string) (*Report, error) {
	out, err := r.Run("adb", "-s", serial, "shell", "getprop")
	if err != nil {
		return nil, fmt.Errorf("getprop: %w", err)
	}
	rep := &Report{Props: ParseGetprop(out)}
	if len(rep.Props) == 0 {
		return nil, fmt.Errorf("getprop não devolveu nenhuma propriedade conhecida")
	}
	if k, err := r.Run("adb", "-s", serial, "shell", "uname", "-r"); err == nil {
		rep.Kernel = strings.TrimSpace(k)
	}
	// Pode falhar sem root: não é erro.
	if m, err := r.Run("adb", "-s", serial, "shell", "ls", "/vendor/lib/modules"); err == nil {
		for _, f := range strings.Fields(m) {
			if strings.HasSuffix(f, ".ko") && len(rep.Modules) < 512 {
				rep.Modules = append(rep.Modules, f)
			}
		}
		sort.Strings(rep.Modules)
	}
	return rep, nil
}

// State é o resultado de um critério: sim, não ou ainda sem dado.
type State string

const (
	Yes     State = "sim"
	No      State = "não"
	Unknown State = "desconhecido"
)

// Finding é um critério avaliado, com a razão.
type Finding struct {
	Criterion string `json:"criterion"`
	State     State  `json:"state"`
	Reason    string `json:"reason"`
}

// KernelVersion extrai "5.10" de "5.10.198-android12-9-g...".
func KernelVersion(release string) (major, minor int, ok bool) {
	m := regexp.MustCompile(`^(\d+)\.(\d+)`).FindStringSubmatch(strings.TrimSpace(release))
	if m == nil {
		return 0, 0, false
	}
	major, _ = strconv.Atoi(m[1])
	minor, _ = strconv.Atoi(m[2])
	return major, minor, true
}

// Assess avalia o que os dados coletados permitem dizer. C2 (fonte GPL) e C3 (device tree) não
// dependem do aparelho e ficam sempre "desconhecido" aqui: precisam de fonte pública verificada.
func (r *Report) Assess() []Finding {
	p := r.Props
	var f []Finding

	switch p["ro.boot.flash.locked"] {
	case "0":
		f = append(f, Finding{"C1 bootloader", Yes, "ro.boot.flash.locked=0: já está destravado"})
	case "1":
		f = append(f, Finding{"C1 bootloader", Unknown, "travado agora; depende de o fabricante permitir destravar (Opções do desenvolvedor → Desbloqueio de OEM; fastboot flashing unlock)"})
	default:
		f = append(f, Finding{"C1 bootloader", Unknown, "o aparelho não informou o estado do bootloader"})
	}
	f = append(f,
		Finding{"C2 fonte GPL do kernel", Unknown, "precisa de fonte pública verificada para este modelo"},
		Finding{"C3 device tree", Unknown, "precisa de repositório público verificado para este modelo"})

	if maj, min, ok := KernelVersion(r.Kernel); ok {
		if maj > 5 || (maj == 5 && min >= 10) {
			f = append(f, Finding{"C4 kernel", Yes, fmt.Sprintf("kernel %d.%d: compatível com o modelo de módulos do ADR-009", maj, min)})
		} else {
			f = append(f, Finding{"C4 kernel", No, fmt.Sprintf("kernel %d.%d é anterior a 5.10 (legado sem GKI): só pacote de driver em userspace", maj, min)})
		}
	} else {
		f = append(f, Finding{"C4 kernel", Unknown, "versão do kernel não lida"})
	}

	switch {
	case p["ro.boot.slot_suffix"] != "":
		f = append(f, Finding{"C5 recovery", Unknown, "tem partições A/B (slot " + p["ro.boot.slot_suffix"] + "), mas o recovery ainda não foi testado"})
	default:
		f = append(f, Finding{"C5 recovery", Unknown, "sem A/B: um flash ruim pode deixar o aparelho sem boot; o recovery precisa ser testado antes"})
	}
	return f
}

// JSON serializa o relatório com a avaliação.
func (r *Report) JSON() ([]byte, error) {
	return json.MarshalIndent(struct {
		*Report
		Assessment []Finding `json:"assessment"`
	}{r, r.Assess()}, "", "  ")
}
