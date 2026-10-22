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
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/plan"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/release"
)

type list []string

func (l *list) String() string     { return strings.Join(*l, ",") }
func (l *list) Set(v string) error { *l = append(*l, v); return nil }

const usage = `uso:
  conexus-installer install [opções]   detecta o aparelho, baixa o core, mostra o plano e instala
  conexus-installer plan    [opções]   mostra o plano sem tocar em aparelho (usa um core já extraído)

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
