// Package main gera, a partir do cadastro de dispositivos do LineageOS (lineage_wiki/_data/devices),
// o conteúdo de um repositório por marca: descritor, tabela de aparelhos e fontes.
//
// Só usa fatos do cadastro (codinome, nome, SoC, repositórios, versão do kernel). Não copia
// textos nem imagens da wiki, nem código de kernel, nem blobs. Os critérios C1–C5 do ADR-010
// saem como DICAS ("a verificar", "declarado"), nunca como evidência verificada.
package main

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

// SoC aceita um texto ou uma lista (alguns aparelhos têm duas variantes de SoC); a lista vira "A / B".
type SoC string

func (s *SoC) UnmarshalYAML(n *yaml.Node) error {
	switch n.Kind {
	case yaml.ScalarNode:
		*s = SoC(n.Value)
	case yaml.SequenceNode:
		parts := make([]string, 0, len(n.Content))
		for _, c := range n.Content {
			if c.Kind != yaml.ScalarNode {
				return fmt.Errorf("soc: item da lista não é texto (linha %d)", c.Line)
			}
			parts = append(parts, c.Value)
		}
		*s = SoC(strings.Join(parts, " / "))
	default:
		return fmt.Errorf("soc: esperava texto ou lista (linha %d)", n.Line)
	}
	return nil
}

// Device é um aparelho do cadastro.
type Device struct {
	Codename      string   `yaml:"codename"`
	Name          string   `yaml:"name"`
	Vendor        string   `yaml:"vendor"`
	VendorShort   string   `yaml:"vendor_short"`
	SoC           SoC      `yaml:"soc"`
	Tree          string   `yaml:"tree"`
	InstallMethod string   `yaml:"install_method"`
	UnlockCmd     string   `yaml:"custom_unlock_cmd"`
	Versions      []string `yaml:"versions"`
	IsAB          *bool    `yaml:"is_ab_device"`
	Kernel        struct {
		Repo    string `yaml:"repo"`
		Version string `yaml:"version"`
	} `yaml:"kernel"`
	Source string `yaml:"-"` // arquivo de origem, relativo ao cadastro
}

// Load lê todos os .yml de um diretório. Arquivo ilegível é erro: não se gera catálogo pela metade.
func Load(dir string) ([]Device, error) {
	files, err := filepath.Glob(filepath.Join(dir, "*.yml"))
	if err != nil {
		return nil, err
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("nenhum .yml em %s", dir)
	}
	sort.Strings(files)
	var out []Device
	for _, f := range files {
		b, err := os.ReadFile(f)
		if err != nil {
			return nil, err
		}
		var d Device
		if err := yaml.Unmarshal(b, &d); err != nil {
			return nil, fmt.Errorf("%s: %w", filepath.Base(f), err)
		}
		if d.Codename == "" || d.Name == "" || d.Vendor == "" {
			return nil, fmt.Errorf("%s: faltam codename, name ou vendor", filepath.Base(f))
		}
		d.Source = filepath.Base(f)
		out = append(out, d)
	}
	return out, nil
}

// Supports informa se o aparelho tem build oficial da versão (ex.: "23.2").
func (d Device) Supports(version string) bool {
	for _, v := range d.Versions {
		if v == version {
			return true
		}
	}
	return false
}

// KernelAtLeast510 devolve "sim", "não" ou "desconhecido" para o critério C4 do ADR-010.
func (d Device) KernelAtLeast510() string {
	parts := strings.Split(strings.TrimSpace(d.Kernel.Version), ".")
	if len(parts) < 2 {
		return "desconhecido"
	}
	maj, e1 := strconv.Atoi(parts[0])
	min, e2 := strconv.Atoi(parts[1])
	if e1 != nil || e2 != nil {
		return "desconhecido"
	}
	if maj > 5 || (maj == 5 && min >= 10) {
		return "sim"
	}
	return "não"
}

// Brand liga o vendor do cadastro ao identificador de marca do projeto. Vazio = fora do escopo.
func Brand(d Device) string {
	switch strings.ToLower(strings.TrimSpace(d.VendorShort)) {
	case "google", "nothing", "oneplus", "samsung", "xiaomi", "motorola", "realme":
		return strings.ToLower(d.VendorShort)
	}
	return ""
}

// SubBrand identifica a linha dentro da marca (POCO, Redmi, Moto G…). Vazio se não houver.
func SubBrand(brand string, d Device) string {
	n := strings.ToLower(d.Name)
	switch brand {
	case "xiaomi":
		switch {
		case strings.Contains(n, "poco"):
			return "poco"
		case strings.Contains(n, "redmi"):
			return "redmi"
		case strings.HasPrefix(n, "mi "):
			return "mi"
		}
	case "motorola":
		switch {
		case strings.Contains(n, "moto g"):
			return "moto-g"
		case strings.Contains(n, "edge"):
			return "edge"
		}
	}
	return ""
}

// Group separa os aparelhos que têm build oficial da versão por marca do projeto e conta o resto.
func Group(devs []Device, version string) (map[string][]Device, int) {
	g := map[string][]Device{}
	skipped := 0
	for _, d := range devs {
		if !d.Supports(version) {
			continue
		}
		b := Brand(d)
		if b == "" {
			skipped++
			continue
		}
		g[b] = append(g[b], d)
	}
	for _, l := range g {
		sort.Slice(l, func(i, j int) bool { return l[i].Codename < l[j].Codename })
	}
	return g, skipped
}
