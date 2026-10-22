# conexus-installer (Go)

Instalador do Conexus Mobile OS. Detecta o aparelho, instala **só o core do sistema** e, **se você escolher**, GApps, APKs e pacotes de driver. Substitui o instalador em Python (`scripts/install_conexus.py`) quando estiver validado em aparelho.

```bash
cd tools/installer
go build -o conexus-installer ./cmd/conexus-installer

# ver o plano sem tocar em aparelho (usa um core já extraído)
./conexus-installer plan --device akita --serial SERIAL --core-dir ./images \
    --gapps ~/gapps.zip --apk ~/app.apk

# instalar (detecta o aparelho, baixa e verifica o core, pede confirmação)
./conexus-installer install --channel stable --gapps ~/gapps.zip --apk ~/app.apk
```

## O que é automático e o que você escolhe

| Item | Quem decide |
|---|---|
| Core do sistema (`boot`, `vbmeta`, `super`, …) | Automático: baixado da release, com SHA-256 do `SHA256SUMS` |
| **GApps** | Você fornece o `.zip` (`--gapps`). O Conexus não o distribui. Instalado no recovery **antes do primeiro boot** |
| **APKs** | Você escolhe (`--apk`, repetível). Instalados com `adb install` depois do boot |
| **Drivers** | Você escolhe o pacote (`--driver`). Só para marca `verified`; senão exige `--experimental-drivers`. Instalados por último. O `conexus-driver-manager` ainda recusa pacote sem assinatura válida |

## Garantias (testadas)

- Nada é executado antes de você digitar `FLASH <codinome> <serial>`.
- `fastboot getvar product` precisa bater com o codinome detectado, senão nada é gravado.
- Sem `SHA256SUMS` válido na release, nada é baixado nem usado. O pacote só pode conter `.img`, `.txt` e `.json`.
- Mais de um aparelho conectado, ou bootloader travado: recusa.
- A primeira falha para tudo. `adb` e `fastboot` são chamados com argumentos fixos, nunca por shell.

## Limites conhecidos

- **Nunca rodou em aparelho** e não há release publicada. Os testes usam um executor falso e um servidor HTTP local.
- Só verifica integridade (SHA-256), não autenticidade: a assinatura de release depende da chave do A5.
- O GApps no recovery tem um passo manual (Apply update → Apply from ADB). A ordem (GApps antes do primeiro boot) segue a prática do LineageOS, mas não foi verificada neste projeto.
- A lista de partições vem do instalador em Python e não foi validada contra uma release real do Pixel 8a.
- Cobre os aparelhos que o core suportar; hoje só existe o alvo Pixel 8a (`akita`) e Nothing Phone (2) (`pong`) como candidatos.

```bash
go vet ./... && go test ./...
```
