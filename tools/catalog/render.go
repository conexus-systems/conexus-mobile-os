package main

import (
	"encoding/json"
	"fmt"
	"sort"
	"strings"
)

// Meta registra de onde os dados vieram.
type Meta struct {
	Version    string // versão do LineageOS, ex.: 23.2
	WikiCommit string // commit do lineage_wiki lido
	Date       string // AAAA-MM-DD
}

const maxDevicesPerDescriptor = 256

type descriptorDevice struct {
	Codename string `json:"codename"`
	Model    string `json:"model,omitempty"`
}

type descriptor struct {
	Brand     string             `json:"brand"`
	SubBrands []string           `json:"sub_brands,omitempty"`
	Status    string             `json:"status"`
	Devices   []descriptorDevice `json:"devices,omitempty"`
}

type deviceFacts struct {
	Codename      string   `json:"codename"`
	Name          string   `json:"name"`
	SubBrand      string   `json:"sub_brand,omitempty"`
	SoC           string   `json:"soc"`
	LineageTree   string   `json:"lineage_device_tree"`
	KernelRepo    string   `json:"lineage_kernel_repo"`
	KernelVersion string   `json:"kernel_version"`
	IsAB          string   `json:"is_ab"`
	InstallMethod string   `json:"install_method"`
	UnlockCmd     string   `json:"unlock_command,omitempty"`
	Versions      []string `json:"lineage_versions"`
	Source        string   `json:"source"`
	Hints         hints    `json:"criteria_hints"`
}

// hints NÃO são evidência: dizem o que o cadastro declara e o que falta verificar.
type hints struct {
	C1 string `json:"c1_unlock"`
	C2 string `json:"c2_kernel_gpl"`
	C3 string `json:"c3_device_tree"`
	C4 string `json:"c4_kernel_ge_5_10"`
	C5 string `json:"c5_recovery"`
}

func abText(b *bool) string {
	switch {
	case b == nil:
		return "desconhecido"
	case *b:
		return "sim"
	}
	return "não"
}

func facts(brand string, d Device, m Meta) deviceFacts {
	c2, c3 := "a verificar", "a verificar"
	if d.Kernel.Repo != "" {
		c2 = "repositório declarado pelo LineageOS: " + d.Kernel.Repo + " (licença e fonte a conferir)"
	}
	if d.Tree != "" {
		c3 = "repositório declarado pelo LineageOS: " + d.Tree
	}
	return deviceFacts{
		Codename: d.Codename, Name: d.Name, SubBrand: SubBrand(brand, d), SoC: string(d.SoC),
		LineageTree: d.Tree, KernelRepo: d.Kernel.Repo, KernelVersion: d.Kernel.Version,
		IsAB: abText(d.IsAB), InstallMethod: d.InstallMethod, UnlockCmd: d.UnlockCmd,
		Versions: d.Versions, Source: "LineageOS/lineage_wiki@" + m.WikiCommit + ":_data/devices/" + d.Source,
		Hints: hints{
			C1: "a verificar: o cadastro indica o método de instalação '" + d.InstallMethod + "', não garante desbloqueio sem espera ou recusa",
			C2: c2, C3: c3, C4: d.KernelAtLeast510(), C5: "não testado",
		},
	}
}

func marshal(v any) string {
	b, _ := json.MarshalIndent(v, "", "  ")
	return string(b) + "\n"
}

// Render devolve os arquivos do repositório de uma marca (caminho → conteúdo), de forma determinística.
func Render(brand string, devs []Device, m Meta) (map[string]string, error) {
	if len(devs) > maxDevicesPerDescriptor {
		return nil, fmt.Errorf("%s: %d aparelhos passam do limite de %d do descritor", brand, len(devs), maxDevicesPerDescriptor)
	}
	desc := descriptor{Brand: brand, Status: "planned"}
	subs := map[string]bool{}
	var all []deviceFacts
	for _, d := range devs {
		if s := SubBrand(brand, d); s != "" {
			subs[s] = true
		}
		desc.Devices = append(desc.Devices, descriptorDevice{Codename: d.Codename, Model: d.Name})
		all = append(all, facts(brand, d, m))
	}
	if len(devs) > 0 {
		desc.Status = "unverified"
	}
	for s := range subs {
		desc.SubBrands = append(desc.SubBrands, s)
	}
	sort.Strings(desc.SubBrands)

	ge510 := 0
	for _, f := range all {
		if f.Hints.C4 == "sim" {
			ge510++
		}
	}
	files := map[string]string{
		"brand-module.json": marshal(desc),
		"devices.json":      marshal(all),
		"CATALOG.md":        catalog(brand, all, m, ge510),
		"SOURCES.md":        sources(m),
		"README.md":         readme(brand, len(all), ge510, m),
		".gitignore":        "# Blobs e arquivos de fabricante NUNCA entram neste repositório (ADR-014).\n/vendor/\n*.ko\n*.so\n*.bin\n*.img\n*.mbn\n*.elf\n*.apk\n*.zip\n*-vendor/\n",
	}
	return files, nil
}

func catalog(brand string, fs []deviceFacts, m Meta, ge510 int) string {
	var b strings.Builder
	fmt.Fprintf(&b, "# Catálogo %s — LineageOS %s\n\nGerado em %s a partir do cadastro de dispositivos do LineageOS (ver SOURCES.md). **Não é verificação**: é o que o cadastro declara.\n\n", brand, m.Version, m.Date)
	if len(fs) == 0 {
		b.WriteString("Nenhum aparelho desta marca tem build oficial do LineageOS " + m.Version + " no cadastro.\n")
		return b.String()
	}
	fmt.Fprintf(&b, "%d aparelhos; %d com kernel ≥ 5.10 (critério C4 do ADR-010).\n\n", len(fs), ge510)
	b.WriteString("| Codinome | Nome | Linha | SoC | Kernel | A/B | Instalação | C4 (kernel ≥ 5.10) |\n|---|---|---|---|---|---|---|---|\n")
	for _, f := range fs {
		fmt.Fprintf(&b, "| `%s` | %s | %s | %s | %s | %s | %s | %s |\n", f.Codename, f.Name, f.SubBrand, f.SoC, f.KernelVersion, f.IsAB, f.InstallMethod, f.Hints.C4)
	}
	return b.String()
}

func sources(m Meta) string {
	return fmt.Sprintf(`# Fontes

- Cadastro de dispositivos: [LineageOS/lineage_wiki](https://github.com/LineageOS/lineage_wiki), pasta `+"`_data/devices`"+`, commit `+"`%s`"+`, lido em %s.
- Só fatos foram usados (codinome, nome, SoC, nomes de repositórios, versão do kernel, método de instalação, versões do LineageOS). Nenhum texto, imagem ou instrução da wiki foi copiado.
- **Licença dos dados:** o repositório da wiki não declara claramente a licença do conteúdo (o `+"`licenses/LICENSE`"+` dele é do tema Jekyll). Atribuição ao projeto LineageOS mantida aqui. Revisão de licença pelo A5 pendente.
- Os repositórios de kernel e de device tree são do LineageOS e dos fabricantes, com as licenças próprias deles (kernel: GPL). **Nada deles foi copiado para cá**, só referenciado pelo nome.
`, m.WikiCommit, m.Date)
}

func readme(brand string, n, ge510 int, m Meta) string {
	state := fmt.Sprintf("%d aparelhos com build oficial do LineageOS %s, %d deles com kernel ≥ 5.10.", n, m.Version, ge510)
	if n == 0 {
		state = "Nenhum aparelho desta marca tem build oficial do LineageOS " + m.Version + " no cadastro. Marca planejada, sem base."
	}
	return fmt.Sprintf(`# conexus-drivers-%s

Catálogo e módulos de driver da marca **%s** para o [Conexus Mobile OS](https://github.com/conexus-systems/conexus-mobile-os).

%s

## O que este repositório é, e o que não é

- **É** um catálogo (`+"`devices.json`"+`, `+"`CATALOG.md`"+`) e o lugar onde ficam os módulos de driver da marca.
- **Não contém drivers.** Para os aparelhos com LineageOS oficial, o kernel, o device tree e os blobs já vêm nos repositórios do LineageOS, e a imagem oficial já os inclui. Aqui só se referencia o nome desses repositórios.
- **Não contém blobs de fabricante** (ADR-014). O `+"`.gitignore`"+` bloqueia os tipos binários.
- **Nada aqui está verificado em aparelho.** O estado é `+"`%s`"+` em `+"`brand-module.json`"+`; só passa a `+"`verified`"+` com C1–C5 aprovados, cada um com fonte e data (ADR-010).

Dados de %s (LineageOS %s). Veja SOURCES.md.
`, brand, brand, state, map[bool]string{true: "unverified", false: "planned"}[n > 0], m.Date, m.Version)
}
