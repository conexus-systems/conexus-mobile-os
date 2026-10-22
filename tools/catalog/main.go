package main

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

func main() {
	wiki := flag.String("wiki", "", "pasta _data/devices do lineage_wiki")
	commit := flag.String("wiki-commit", "", "commit do lineage_wiki lido")
	version := flag.String("version", "23.2", "versão do LineageOS")
	date := flag.String("date", "", "data da leitura (AAAA-MM-DD)")
	out := flag.String("out", "", "pasta de saída (uma subpasta por marca)")
	extra := flag.String("planned", "tcl,oppo", "marcas sem dados no cadastro, geradas como planned")
	flag.Parse()
	if *wiki == "" || *commit == "" || *date == "" || *out == "" {
		fmt.Fprintln(os.Stderr, "uso: catalog --wiki DIR --wiki-commit SHA --date AAAA-MM-DD --out DIR [--version 23.2] [--planned tcl,oppo]")
		os.Exit(2)
	}
	devs, err := Load(*wiki)
	if err != nil {
		fmt.Fprintln(os.Stderr, "erro:", err)
		os.Exit(1)
	}
	groups, skipped := Group(devs, *version)
	for _, b := range strings.Split(*extra, ",") {
		if b = strings.TrimSpace(b); b != "" {
			if _, ok := groups[b]; !ok {
				groups[b] = nil
			}
		}
	}
	m := Meta{Version: *version, WikiCommit: *commit, Date: *date}
	var brands []string
	for b := range groups {
		brands = append(brands, b)
	}
	sort.Strings(brands)
	for _, b := range brands {
		files, err := Render(b, groups[b], m)
		if err != nil {
			fmt.Fprintln(os.Stderr, "erro:", err)
			os.Exit(1)
		}
		for name, content := range files {
			p := filepath.Join(*out, b, name)
			if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
				fmt.Fprintln(os.Stderr, "erro:", err)
				os.Exit(1)
			}
			if err := os.WriteFile(p, []byte(content), 0o644); err != nil {
				fmt.Fprintln(os.Stderr, "erro:", err)
				os.Exit(1)
			}
		}
		fmt.Printf("%-9s %3d aparelhos\n", b, len(groups[b]))
	}
	fmt.Printf("fora do escopo (outras marcas com %s): %d aparelhos\n", *version, skipped)
}
