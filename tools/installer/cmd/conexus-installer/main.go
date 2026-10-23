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
