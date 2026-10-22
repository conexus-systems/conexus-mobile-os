// Package release baixa o core do sistema de uma release do GitHub e só o entrega verificado:
// o hash vem do asset SHA256SUMS da própria release, e nada é usado sem hash válido.
package release

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/conexus-systems/conexus-mobile-os/tools/installer/plan"
	"github.com/conexus-systems/conexus-mobile-os/tools/installer/safety"
)

// DefaultAPI é o repositório oficial.
const DefaultAPI = "https://api.github.com/repos/conexus-systems/conexus-mobile-os"

const maxCoreSize = int64(8) << 30

// ErrNoRelease: não há release publicada para o aparelho e o canal.
var ErrNoRelease = errors.New("nenhuma release encontrada")

// Client busca releases. AllowInsecure existe só para testes com servidor local.
type Client struct {
	API           string
	HTTP          *http.Client
	AllowInsecure bool
}

type ghRelease struct {
	Tag    string `json:"tag_name"`
	Assets []struct {
		Name string `json:"name"`
		URL  string `json:"browser_download_url"`
		Size int64  `json:"size"`
	} `json:"assets"`
}

func (c *Client) get(ctx context.Context, rawURL string) (*http.Response, error) {
	u, err := url.Parse(rawURL)
	if err != nil {
		return nil, err
	}
	if u.Scheme != "https" && !(c.AllowInsecure && u.Scheme == "http") {
		return nil, fmt.Errorf("somente https é aceito: %s", rawURL)
	}
	req, _ := http.NewRequestWithContext(ctx, http.MethodGet, rawURL, nil)
	req.Header.Set("User-Agent", "conexus-installer")
	hc := c.HTTP
	if hc == nil {
		hc = &http.Client{Timeout: 30 * time.Minute}
	}
	resp, err := hc.Do(req)
	if err != nil {
		return nil, err
	}
	if resp.StatusCode != http.StatusOK {
		resp.Body.Close()
		return nil, fmt.Errorf("HTTP %d em %s", resp.StatusCode, rawURL)
	}
	return resp, nil
}

// FetchCore baixa, verifica e extrai o core do aparelho para destDir.
func (c *Client) FetchCore(ctx context.Context, codename, channel, destDir string) (plan.Core, error) {
	api := c.API
	if api == "" {
		api = DefaultAPI
	}
	resp, err := c.get(ctx, api+"/releases?per_page=30")
	if err != nil {
		return plan.Core{}, err
	}
	var releases []ghRelease
	err = json.NewDecoder(io.LimitReader(resp.Body, 8<<20)).Decode(&releases)
	resp.Body.Close()
	if err != nil {
		return plan.Core{}, err
	}
	for _, r := range releases {
		if !strings.Contains(r.Tag, codename) || !strings.Contains(r.Tag, channel) {
			continue
		}
		var zipName, zipURL, sumsURL string
		var zipSize int64
		for _, a := range r.Assets {
			switch {
			case a.Name == "SHA256SUMS":
				sumsURL = a.URL
			case strings.HasSuffix(a.Name, ".zip") && strings.Contains(a.Name, codename):
				zipName, zipURL, zipSize = a.Name, a.URL, a.Size
			}
		}
		if sumsURL == "" {
			return plan.Core{}, fmt.Errorf("a release %s não tem SHA256SUMS; recusada", r.Tag)
		}
		if zipURL == "" {
			return plan.Core{}, fmt.Errorf("a release %s não tem o pacote do aparelho", r.Tag)
		}
		if zipSize <= 0 || zipSize > maxCoreSize {
			return plan.Core{}, fmt.Errorf("tamanho do pacote inválido: %d", zipSize)
		}
		sresp, err := c.get(ctx, sumsURL)
		if err != nil {
			return plan.Core{}, err
		}
		sums, err := io.ReadAll(io.LimitReader(sresp.Body, 1<<20))
		sresp.Body.Close()
		if err != nil {
			return plan.Core{}, err
		}
		want := safety.ParseSHA256Sums(string(sums))[zipName]
		if !safety.IsValidSHA256(want) {
			return plan.Core{}, fmt.Errorf("SHA256SUMS não tem hash válido para %s", zipName)
		}
		return c.download(ctx, r.Tag, zipURL, zipName, want, destDir)
	}
	return plan.Core{}, fmt.Errorf("%w para %s no canal %s", ErrNoRelease, codename, channel)
}

func (c *Client) download(ctx context.Context, tag, zipURL, zipName, want, destDir string) (plan.Core, error) {
	if err := os.MkdirAll(destDir, 0o700); err != nil {
		return plan.Core{}, err
	}
	zpath := filepath.Join(destDir, filepath.Base(zipName))
	resp, err := c.get(ctx, zipURL)
	if err != nil {
		return plan.Core{}, err
	}
	defer resp.Body.Close()
	f, err := os.OpenFile(zpath, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0o600)
	if err != nil {
		return plan.Core{}, err
	}
	n, err := io.Copy(f, io.LimitReader(resp.Body, maxCoreSize+1))
	f.Close()
	if err != nil {
		return plan.Core{}, err
	}
	if n > maxCoreSize {
		return plan.Core{}, errors.New("pacote maior que o limite")
	}
	if err := safety.VerifySHA256(zpath, want); err != nil {
		os.Remove(zpath)
		return plan.Core{}, err
	}
	files, err := safety.SafeExtract(zpath, filepath.Join(destDir, "images"), safety.DefaultMaxTotal)
	if err != nil {
		return plan.Core{}, err
	}
	if err := safety.RequireImages(files); err != nil {
		return plan.Core{}, err
	}
	images := map[string]string{}
	for _, p := range files {
		if base := filepath.Base(p); strings.HasSuffix(base, ".img") {
			images[strings.TrimSuffix(base, ".img")] = p
		}
	}
	return plan.Core{Version: tag, Images: images}, nil
}
