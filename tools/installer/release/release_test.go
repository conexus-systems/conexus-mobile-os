package release

import (
	"archive/zip"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func zipBytes(entries map[string]string) []byte {
	var b bytes.Buffer
	zw := zip.NewWriter(&b)
	for n, d := range entries {
		w, _ := zw.Create(n)
		w.Write([]byte(d))
	}
	zw.Close()
	return b.Bytes()
}

// server monta uma "release" falsa. sums controla o conteúdo do SHA256SUMS.
func server(t *testing.T, tag string, zipData []byte, sums func(name string, sum string) string, withSums bool) *httptest.Server {
	t.Helper()
	var srv *httptest.Server
	mux := http.NewServeMux()
	mux.HandleFunc("/releases", func(w http.ResponseWriter, r *http.Request) {
		assets := fmt.Sprintf(`{"name":"conexus-akita.zip","browser_download_url":"%s/dl/zip","size":%d}`, srv.URL, len(zipData))
		if withSums {
			assets += fmt.Sprintf(`,{"name":"SHA256SUMS","browser_download_url":"%s/dl/sums","size":10}`, srv.URL)
		}
		fmt.Fprintf(w, `[{"tag_name":%q,"assets":[%s]}]`, tag, assets)
	})
	mux.HandleFunc("/dl/zip", func(w http.ResponseWriter, r *http.Request) { w.Write(zipData) })
	mux.HandleFunc("/dl/sums", func(w http.ResponseWriter, r *http.Request) {
		h := sha256.Sum256(zipData)
		fmt.Fprint(w, sums("conexus-akita.zip", hex.EncodeToString(h[:])))
	})
	srv = httptest.NewServer(mux)
	t.Cleanup(srv.Close)
	return srv
}

func goodSums(name, sum string) string { return sum + "  " + name + "\n" }

var images = map[string]string{"boot.img": "b", "vbmeta.img": "v", "super.img": "s", "dtbo.img": "d"}

func TestFetchCoreVerifiesAndExtracts(t *testing.T) {
	srv := server(t, "v1.0.0-stable-akita", zipBytes(images), goodSums, true)
	c := &Client{API: srv.URL, AllowInsecure: true}
	core, err := c.FetchCore(context.Background(), "akita", "stable", t.TempDir())
	if err != nil || core.Images["boot"] == "" || core.Images["super"] == "" || core.Version != "v1.0.0-stable-akita" {
		t.Fatalf("inesperado: %+v %v", core, err)
	}
}

func TestFetchCoreRefusesWrongHash(t *testing.T) {
	srv := server(t, "v1-stable-akita", zipBytes(images), func(n, _ string) string { return strings.Repeat("a", 64) + "  " + n + "\n" }, true)
	c := &Client{API: srv.URL, AllowInsecure: true}
	if _, err := c.FetchCore(context.Background(), "akita", "stable", t.TempDir()); err == nil || !strings.Contains(err.Error(), "SHA-256") {
		t.Fatalf("hash errado deveria ser recusado: %v", err)
	}
}

func TestFetchCoreRefusesMissingSumsAndMissingHashEntry(t *testing.T) {
	srv := server(t, "v1-stable-akita", zipBytes(images), goodSums, false)
	if _, err := (&Client{API: srv.URL, AllowInsecure: true}).FetchCore(context.Background(), "akita", "stable", t.TempDir()); err == nil {
		t.Error("release sem SHA256SUMS deveria ser recusada")
	}
	srv2 := server(t, "v1-stable-akita", zipBytes(images), func(string, string) string { return "" }, true)
	if _, err := (&Client{API: srv2.URL, AllowInsecure: true}).FetchCore(context.Background(), "akita", "stable", t.TempDir()); err == nil {
		t.Error("SHA256SUMS sem entrada deveria ser recusado")
	}
}

func TestFetchCoreRefusesPackageWithoutRequiredImagesOrWithTraversal(t *testing.T) {
	for name, entries := range map[string]map[string]string{
		"sem vbmeta": {"boot.img": "b", "super.img": "s"},
		"traversal":  {"../x.img": "x", "boot.img": "b", "vbmeta.img": "v", "super.img": "s"},
	} {
		srv := server(t, "v1-stable-akita", zipBytes(entries), goodSums, true)
		if _, err := (&Client{API: srv.URL, AllowInsecure: true}).FetchCore(context.Background(), "akita", "stable", t.TempDir()); err == nil {
			t.Errorf("%s deveria ser recusado", name)
		}
	}
}

func TestNoReleaseAndInsecureURL(t *testing.T) {
	srv := server(t, "v1-stable-pong", zipBytes(images), goodSums, true)
	if _, err := (&Client{API: srv.URL, AllowInsecure: true}).FetchCore(context.Background(), "akita", "stable", t.TempDir()); !errors.Is(err, ErrNoRelease) {
		t.Errorf("sem release para o aparelho: %v", err)
	}
	if _, err := (&Client{API: srv.URL}).FetchCore(context.Background(), "akita", "stable", t.TempDir()); err == nil || !strings.Contains(err.Error(), "https") {
		t.Errorf("http deveria ser recusado: %v", err)
	}
}
