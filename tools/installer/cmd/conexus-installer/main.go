// conexus-installer detecta o aparelho, instala o core do sistema e, só se você escolher,
// GApps, APKs e pacotes de driver. Nada além do core é instalado por padrão.
package main

import (
	"bufio"
	"context"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/apply"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/device"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/extract"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/inventory"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/plan"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/release"
)

type list []string

func (l *list) String() string     { return strings.Join(*l, ",") }
func (l *list) Set(v string) error { *l = append(*l, v); return nil }

const usage = `uso:
  conexus-installer install [opções]   detecta o aparelho, baixa o core, mostra o plano e instala
  conexus-installer plan    [opções]   mostra o plano sem tocar em aparelho (usa um core já extraído)
  conexus-installer extract --out PASTA_NOVA   copia do SEU aparelho os arquivos de vendor (módulos, HALs, firmware)
                                       para construir o módulo de drivers dele. Só lê. Não publique o resultado.
  conexus-installer inventory [--out arquivo.json]   só lê o aparelho (adb ligado) e avalia os critérios C1–C5;
                                       não grava nada e não coleta serial nem IMEI
  conexus-installer unlock-check                 verifica se o bootloader pode ser destravado (OEM unlock),
                                       guia o processo e testa fastboot flashing unlock

extras que você escolhe (nada disso vem por padrão):
  --gapps arquivo.zip        GApps fornecido por você (o Conexus não distribui)
  --apk arquivo.apk          APK comum (repetível)
  --driver arquivo.apk       pacote de driver (repetível); marcas sem verificação exigem --experimental-drivers

opções:
  --channel stable|monthly|weekly|nightly   (install; padrão stable)
  --brands-dir pasta                         descritores brands/*.json
  --dry-run                                  (install) mostra o que faria e não executa
  --core-dir pasta --device codinome --serial serial   (plan)
`

func main() {
	if len(os.Args) < 2 {
		fmt.Fprint(os.Stderr, usage)
		os.Exit(2)
	}
	cmd, args := os.Args[1], os.Args[2:]
	fs := flag.NewFlagSet(cmd, flag.ExitOnError)
	var apks, drivers list
	gapps := fs.String("gapps", "", "")
	fs.Var(&apks, "apk", "")
	fs.Var(&drivers, "driver", "")
	experimental := fs.Bool("experimental-drivers", false, "")
	channel := fs.String("channel", "stable", "")
	brandsDir := fs.String("brands-dir", "", "")
	dry := fs.Bool("dry-run", false, "")
	coreDir := fs.String("core-dir", "", "")
	devName := fs.String("device", "", "")
	serial := fs.String("serial", "", "")
	out := fs.String("out", "", "")
	fs.Usage = func() { fmt.Fprint(os.Stderr, usage) }
	fs.Parse(args)

	var brands []plan.BrandInfo
	if *brandsDir != "" {
		var err error
		if brands, err = plan.LoadBrands(*brandsDir); err != nil {
			fatal(err)
		}
	}
	opts := plan.Options{GApps: *gapps, APKs: apks, DriverBundles: drivers, ExperimentalDrivers: *experimental}

	switch cmd {
	case "extract":
		if *out == "" {
			fatal(fmt.Errorf("extract exige --out com uma pasta nova"))
		}
		runner := device.ExecRunner{}
		dev, err := device.Detect(runner)
		if err != nil {
			fatal(err)
		}
		if dev.Mode != device.ModeADB {
			fatal(fmt.Errorf("a extração precisa do aparelho ligado, com a depuração USB autorizada (modo adb)"))
		}
		m, err := extract.Run(runner, dev.Serial, *out)
		if m != nil {
			fmt.Printf("%d arquivos copiados para %s\n", len(m.Files), *out)
			for _, s := range m.Skipped {
				fmt.Println("  não lido (sem permissão ou inexistente):", s)
			}
		}
		if err != nil {
			fatal(err)
		}
		fmt.Println("ATENÇÃO: são arquivos do fabricante. Não os publique nem os envie ao repositório.")
	case "inventory":
		runner := device.ExecRunner{}
		dev, err := device.Detect(runner)
		if err != nil {
			fatal(err)
		}
		if dev.Mode != device.ModeADB {
			fatal(fmt.Errorf("o inventário precisa do aparelho ligado, com a depuração USB autorizada (modo adb)"))
		}
		rep, err := inventory.Collect(runner, dev.Serial)
		if err != nil {
			fatal(err)
		}
		fmt.Printf("%s %s — SoC %s — kernel %s\n", rep.Props["ro.product.manufacturer"], rep.Props["ro.product.model"], rep.Props["ro.board.platform"], rep.Kernel)
		for _, f := range rep.Assess() {
			fmt.Printf("  %-26s %-13s %s\n", f.Criterion, f.State, f.Reason)
		}
		if *out != "" {
			b, err := rep.JSON()
			if err != nil {
				fatal(err)
			}
			if err := os.WriteFile(*out, b, 0o600); err != nil {
				fatal(err)
			}
			fmt.Println("Relatório salvo em", *out)
		}
	case "plan":
		if *coreDir == "" || *devName == "" || *serial == "" {
			fatal(fmt.Errorf("plan exige --core-dir, --device e --serial"))
		}
		core, err := coreFromDir(*coreDir)
		if err != nil {
			fatal(err)
		}
		unlocked := true
		p, err := plan.Build(device.Device{Serial: *serial, Codename: *devName, Mode: device.ModeFastboot, BootloaderUnlocked: &unlocked}, core, opts, brands)
		if err != nil {
			fatal(err)
		}
		printPlan(p)
	case "install":
		runner := device.ExecRunner{}
		dev, err := device.Detect(runner)
		if err != nil {
			fatal(err)
		}
		fmt.Printf("Aparelho: %s (%s), modo %s\n", dev.Codename, dev.Serial, dev.Mode)
		tmp, err := os.MkdirTemp("", "conexus-install-")
		if err != nil {
			fatal(err)
		}
		defer os.RemoveAll(tmp)
		fmt.Println("Baixando e verificando o core...")
		core, err := (&release.Client{}).FetchCore(context.Background(), dev.Codename, *channel, tmp)
		if err != nil {
			fatal(err)
		}
		p, err := plan.Build(*dev, core, opts, brands)
		if err != nil {
			fatal(err)
		}
		printPlan(p)
		in := bufio.NewReader(os.Stdin)
		read := func(msg string) string {
			fmt.Printf("\n%s\n> ", msg)
			s, _ := in.ReadString('\n')
			return strings.TrimSpace(s)
		}
		err = apply.Execute(p, apply.Options{
			Runner: runner, DryRun: *dry,
			Prompt: read,
			Pause:  func(msg string) { read(msg) },
			Log:    func(f string, a ...any) { fmt.Printf(f+"\n", a...) },
		})
		if err != nil {
			fatal(err)
		}
		fmt.Println("Concluído.")
	case "unlock-check":
		runner := device.ExecRunner{}
		if err := runUnlockCheck(runner); err != nil {
			fatal(err)
		}
	default:
		fmt.Fprint(os.Stderr, usage)
		os.Exit(2)
	}
}

func coreFromDir(dir string) (plan.Core, error) {
	files, err := filepath.Glob(filepath.Join(dir, "*.img"))
	if err != nil || len(files) == 0 {
		return plan.Core{}, fmt.Errorf("nenhum .img em %s", dir)
	}
	images := map[string]string{}
	for _, f := range files {
		images[strings.TrimSuffix(filepath.Base(f), ".img")] = f
	}
	return plan.Core{Version: "local", Images: images}, nil
}

func printPlan(p *plan.Plan) {
	fmt.Printf("\n== Plano para %s (%s) — core %s ==\n", p.Device.Codename, p.Device.Serial, p.Core.Version)
	for i, s := range p.Steps {
		detail := strings.Join(s.Cmd, " ")
		if s.Manual != "" {
			detail = "[manual] " + s.Manual
		}
		fmt.Printf("%2d. [%s] %s  %s\n", i+1, s.Phase, s.Title, detail)
	}
	if len(p.Extras) > 0 {
		fmt.Println("\nExtras escolhidos:")
		for _, e := range p.Extras {
			fmt.Printf("  - %s %s (sha256 %s)\n", e.Kind, e.Path, e.SHA256)
		}
	}
	for _, w := range p.Warnings {
		fmt.Println("AVISO:", w)
	}
}

func fatal(err error) {
	fmt.Fprintln(os.Stderr, "erro:", err)
	os.Exit(1)
}

// runUnlockCheck verifica o status do OEM unlock e guia o processo de desbloqueio do bootloader
func runUnlockCheck(r device.Runner) error {
	fmt.Println("=== Verificação de Desbloqueio de Bootloader (C1 - ADR-010) ===")
	fmt.Println()

	// 1. Detectar aparelho em modo ADB
	dev, err := device.Detect(r)
	if err != nil {
		return fmt.Errorf("detecção: %w", err)
	}
	if dev.Mode != device.ModeADB {
		return fmt.Errorf("o aparelho precisa estar ligado, com depuração USB autorizada (modo ADB)")
	}

	fmt.Printf("Aparelho detectado: %s %s (%s)\n", dev.Codename, dev.Serial, dev.Mode)
	fmt.Println()

	// 2. Ler propriedades relevantes
	props := map[string]string{}
	propKeys := []string{
		"ro.oem_unlock_supported",
		"ro.boot.flash.locked",
		"ro.boot.verifiedbootstate",
		"ro.boot.vbmeta.device_state",
		"ro.boot.loader_type",
	}
	for _, key := range propKeys {
		out, err := r.Run("adb", "-s", dev.Serial, "shell", "getprop", key)
		if err == nil {
			props[key] = strings.TrimSpace(out)
		}
	}

	fmt.Println("--- Status Atual ---")
	fmt.Printf("  ro.oem_unlock_supported: %s\n", props["ro.oem_unlock_supported"])
	fmt.Printf("  ro.boot.flash.locked:    %s (0=unlocked, 1=locked)\n", props["ro.boot.flash.locked"])
	fmt.Printf("  ro.boot.verifiedbootstate: %s\n", props["ro.boot.verifiedbootstate"])
	fmt.Printf("  ro.boot.vbmeta.device_state: %s\n", props["ro.boot.vmeta.device_state"])
	if props["ro.boot.loader_type"] != "" {
		fmt.Printf("  ro.boot.loader_type:     %s\n", props["ro.boot.loader_type"])
	}
	fmt.Println()

	// 3. Avaliar estado
	oemSupported := props["ro.oem_unlock_supported"] == "1"
	bootloaderLocked := props["ro.boot.flash.locked"] == "1"

	if !bootloaderLocked {
		fmt.Println("✅ BOOTLOADER JÁ DESTRAVADO (ro.boot.flash.locked=0)")
		fmt.Println("   C1: PASS — pode prosseguir para instalação/extração")
		return nil
	}

	if !oemSupported {
		fmt.Println("❌ OEM UNLOCK NÃO SUPORTADO PELO FABRICANTE (ro.oem_unlock_supported=0)")
		fmt.Println("   C1: FAIL — Bootloader permanentemente travado")
		fmt.Println()
		fmt.Println("   Este aparelho NÃO PODE receber sistema customizado.")
		fmt.Println("   Projeto para este dispositivo: ENCERRADO (NO-GO)")
		return nil
	}

	// 4. OEM unlock suportado, mas bootloader travado
	fmt.Println("⚠️  Bootloader travado, mas OEM unlock suportado pelo fabricante")
	fmt.Println("   Próximo passo: ATIVAR 'Desbloqueio de OEM' nas Opções do Desenvolvedor")
	fmt.Println()

	// 5. Verificar se Opções do Desenvolvedor está ativa
	devOptions, _ := r.Run("adb", "-s", dev.Serial, "shell", "settings", "get", "global", "development_settings_enabled")
	devEnabled := strings.TrimSpace(devOptions) == "1"
	if !devEnabled {
		fmt.Println("⚠️  Opções do Desenvolvedor NÃO estão ativas")
		fmt.Println("   Ative em: Configurações > Sobre o telefone > Número da versão (toque 7x)")
		fmt.Println("   Depois: Configurações > Sistema > Opções do desenvolvedor > ON")
		fmt.Println()
	}

	// 6. Guia passo a passo
	fmt.Println("--- Passo a Passo para Desbloqueio ---")
	fmt.Println()
	fmt.Println("1. No telefone:")
	fmt.Println("   Configurações > Sistema > Opções do desenvolvedor")
	fmt.Println("   → Ative 'Depuração USB' (se não estiver)")
	fmt.Println("   → Ative 'Desbloqueio de OEM' (pode pedir PIN/biometria)")
	fmt.Println()
	fmt.Println("2. Conecte via USB (cabo de DADOS, não só carregamento)")
	fmt.Println("   Autorize a depuração USB no popup do telefone")
	fmt.Println()
	fmt.Println("3. Execute o comando de desbloqueio:")
	fmt.Println("   adb reboot bootloader")
	fmt.Println("   fastboot flashing unlock")
	fmt.Println()
	fmt.Println("4. No telefone (tela de bootloader):")
	fmt.Println("   → Use VOLUME +/– para navegar")
	fmt.Println("   → POWER para confirmar 'Unlock the bootloader'")
	fmt.Println("   ⚠️  APAGA TODOS OS DADOS DO APARELHO")
	fmt.Println()
	fmt.Println("5. Após unlock, o telefone fará factory reset e reiniciará")
	fmt.Println("   Configure novamente, reative Depuração USB + Desbloqueio de OEM")
	fmt.Println()
	fmt.Println("6. Verifique se destravou:")
	fmt.Println("   fastboot getvar unlocked")
	fmt.Println("   Deve retornar: 'unlocked: yes'")
	fmt.Println()

	// 7. Oferecer executar teste agora
	fmt.Print("Deseja tentar o desbloqueio AGORA? (y/N): ")
	var response string
	fmt.Scanln(&response)
	if strings.ToLower(strings.TrimSpace(response)) != "y" {
		fmt.Println("Cancelado. Execute 'conexus-installer unlock-check' novamente quando pronto.")
		return nil
	}

	// 8. Executar sequência de unlock
	fmt.Println()
	fmt.Println("=== Executando desbloqueio ===")
	fmt.Println()

	// Reboot to bootloader
	fmt.Println("→ Reiniciando para bootloader...")
	out, err := r.Run("adb", "-s", dev.Serial, "reboot", "bootloader")
	if err != nil {
		return fmt.Errorf("adb reboot bootloader: %w (%s)", err, out)
	}
	// Wait for fastboot
	time.Sleep(3 * time.Second)

	// Check fastboot device
	fbOut, _ := r.Run("fastboot", "devices")
	if !strings.Contains(fbOut, dev.Serial) {
		fmt.Println("⚠️  Aparelho não apareceu em fastboot. Aguardando mais...")
		time.Sleep(5 * time.Second)
		fbOut, _ = r.Run("fastboot", "devices")
	}
	fmt.Printf("Fastboot devices: %s\n", strings.TrimSpace(fbOut))

	// Try flashing unlock
	fmt.Println("→ Enviando 'fastboot flashing unlock'...")
	out, err = r.Run("fastboot", "-s", dev.Serial, "flashing", "unlock")
	fmt.Printf("Saída: %s\n", out)
	if err != nil {
		fmt.Printf("Erro: %v\n", err)
		// Try legacy command
		fmt.Println("→ Tentando comando legado 'fastboot oem unlock'...")
		out, err = r.Run("fastboot", "-s", dev.Serial, "oem", "unlock")
		fmt.Printf("Saída: %s\n", out)
		if err != nil {
			fmt.Printf("Erro: %v\n", err)
		}
	}

	// Check result
	fmt.Println("→ Verificando status...")
	out, _ = r.Run("fastboot", "-s", dev.Serial, "getvar", "unlocked")
	fmt.Printf("fastboot getvar unlocked: %s\n", out)

	if strings.Contains(strings.ToLower(out), "unlocked: yes") {
		fmt.Println()
		fmt.Println("✅ BOOTLOADER DESTRAVADO COM SUCESSO!")
		fmt.Println("   C1: PASS")
		fmt.Println()
		fmt.Println("   Próximos passos:")
		fmt.Println("   1. Telefone fará factory reset e reiniciará")
		fmt.Println("   2. Configure novamente, reative Depuração USB + Desbloqueio de OEM")
		fmt.Println("   3. Rode 'conexus-installer inventory --out tcl505.json' para confirmar")
		fmt.Println("   4. Rode 'conexus-installer extract --out ../tcl505-vendor' para extrair blobs")
		return nil
	}

	fmt.Println()
	fmt.Println("❌ DESBLOQUEIO FALHOU ou não confirmado")
	fmt.Println("   Verifique se:")
	fmt.Println("   - 'Desbloqueio de OEM' estava ATIVADO nas Opções do Desenvolvedor")
	fmt.Println("   - Você confirmou 'Unlock the bootloader' na tela do telefone (POWER)")
	fmt.Println("   - Cabo USB é de dados (teste: 'adb devices' no modo normal)")
	fmt.Println()
	fmt.Println("   Se a opção 'Desbloqueio de OEM' não existe ou está cinza:")
	fmt.Println("   → Bootloader permanentemente travado = C1 FAIL = NO-GO")
	return nil
}
