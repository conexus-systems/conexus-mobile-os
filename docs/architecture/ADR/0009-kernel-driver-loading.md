# ADR-009: Kernel e carregamento de drivers adicionais — **PROPOSTO**

**Status:** Proposto (aguarda aprovação de A0 e revisão de segurança A5)
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Security (A5), Platform (A2)
**Assinatura:** pendente

---

## Contexto

O Tiago pediu um sistema operacional com kernel que permita instalar drivers adicionais. Isso conflita com o SDD atual:

- `SDD.md` (seção de segurança, perfil de produção): `MODULES_DISABLE=1`, `KPTR_RESTRICT=2`, KASLR, GKI 6.1.
- `SDD.md` §1 e `SECURITY.md`: "Kernel próprio do zero" fica fora do escopo.
- `ADR-001`: base AOSP/LineageOS, kernel do fabricante ou GKI do device.
- `SEC-001` (integridade) e `SEC-002` (permissões) são P0; SELinux enforcing e AVB são requisitos de A5.
- O repo já contém `driver-core/` (framework Java com `DriverInstaller`, abstrações por versão de kernel 4.19–6.6). Ainda não foi revisado nem integrado ao build; este ADR deve dizer se ele entra ou sai.

Um driver que roda no kernel tem acesso total à memória. Permitir instalação sem assinatura é o maior risco de segurança do projeto.

---

## Decisão proposta

### Core único, drivers como pacotes

O sistema tem **um core só**: o mesmo conjunto de serviços de sistema (`conexus-*`), o mesmo framework de drivers e a mesma ABI de driver, em todos os alvos. Cada dispositivo adiciona apenas:

- o kernel do próprio device (GKI do Pixel, kernel do fabricante ou Linux do Lumia), porque um único binário de kernel não serve para SoCs diferentes;
- um **pacote de driver** por componente de hardware, com manifest, assinado pela chave de driver (ex.: `motorola-moto-g-touch`, `pixel-8a-camera`).

Um pacote de driver é userspace por padrão (classe 1). Só entra como módulo de kernel quando não há interface userspace (classe 2). Um APK de driver não recebe privilégio de kernel e só é aceito pelo `conexus-driver-manager` se estiver assinado pela chave de driver do projeto.

Consequência para a pergunta de instalar drivers de fabricante: os blobs de um device específico são um pacote de driver desse device, gerado a partir do aparelho do usuário e fora do repo (ver ADR de device, a criar).

**Manter um kernel Linux upstream (GKI do device) e habilitar módulos carregáveis com assinatura obrigatória. Drivers de terceiros entram em duas classes:**

1. **Drivers de userspace (preferidos):** HAL/serviço em processo isolado, comunicação por Binder/AIDL (Android) ou D-Bus (Lumia). Não tocam o kernel. Instalados pelo `conexus-driver-manager`, com domínio SELinux próprio. Cobrem a maior parte dos casos (sensores via IIO userspace, USB gadget, dispositivos com interface VFIO/UIO).
2. **Módulos de kernel (exceção):** só para hardware sem interface de userspace. Requisitos:
   - Compilados contra o KMI da versão exata do kernel instalado (GKI: `vendor_dlkm`; Lumia: kernel do build).
   - Assinados com chave de **driver**, separada das chaves de release e de dev (ADR-005, política de chaves do A5). A chave de release nunca assina módulos.
   - Carregados com `CONFIG_MODULE_SIG_FORCE=y`. Não há modo de bypass em build de release.
   - Carregamento só por um serviço privilegiado, com manifest validado (`framework/sdk/shared/driver-manifest.schema.json`, a criar), versão de kernel e hash.
   - Auditoria em `conexus-security` (SDD §7).

**Perfis de build:**

| Perfil | `CONFIG_MODULES` | Assinatura | Quem instala |
|---|---|---|---|
| `dev` | `y` | Chave dev, aviso na UI | Desenvolvedor, bootloader desbloqueado |
| `alpha` / `beta` | `y` | Chave driver de release | Usuário, com confirmação explícita e log |
| `stable` | `y` | Chave driver de release, `MODULE_SIG_FORCE` | Apenas drivers do Conexus e do fabricante |

O perfil `MODULES_DISABLE=1` deixa de ser o padrão e passa a valer apenas para o modo "imagem travada" (opcional, por build).

---

## Consequências

### Positivas
- Hardware novo pode ser suportado sem reflash completo do sistema.
- Boa parte dos drivers fica fora do kernel, reduzindo a superfície de ataque.
- Mantém compatibilidade com GKI e com o modelo de atualização A/B (ADR-008).

### Negativas
- Mais complexidade no build: módulos por kernel, por device e por KMI.
- Cada atualização de kernel pode quebrar módulos de terceiros. Exige recompilação e nova assinatura.
- Drivers de terceiros com bug podem derrubar o sistema. Mitigação: carregamento em teste, rollback automático de módulo (`rollback protection`, ADR-008).

### Riscos
- **R-KMI:** o GKI restringe símbolos exportados; módulos de terceiros podem não linkar. Mitigação: só usar símbolos do KMI; caso contrário, o driver vira userspace.
- **R-SEC:** chave de driver vazada permite rootkit assinado. Mitigação: chave em HSM/offline, rotação, revogação via `conexus-update`.
- **R-AVB:** módulo em `vendor_dlkm` fora da cadeia verificada. Mitigação: `vendor_dlkm` entra na imagem assinada; módulos instalados pelo usuário ficam em partição de dados, verificados por assinatura no carregamento.
- **R-Lumia:** nos Lumia 950/950 XL o kernel vem do trabalho de UEFI/Linux comunitário, com suporte a módulos a confirmar. Pode haver só userspace.

---

## Alternativas consideradas

| Alternativa | Prós | Contras | Por que não (ou por que sim) |
|---|---|---|---|
| Kernel sem módulos (`MODULES_DISABLE=1`), todo driver compilado na imagem | Superfície mínima, sem assinatura de módulo | Não atende ao pedido de instalar drivers | Rejeitada como padrão; mantida como perfil opcional |
| Só drivers de userspace (sem LKM) | Mais seguro, isolado, sem recompilar kernel | Não cobre hardware sem interface userspace | Adotada como preferência (classe 1) |
| Kernel próprio do zero com driver model próprio | Controle total | Escopo enorme, fora do SDD | Rejeitada (SDD §1) |
| Módulos sem assinatura, em dev e produção | Simples | Quebra integridade e SEC-001 | Rejeitada |
| Usar `driver-core/` (Java) como framework de instalação | Já existe no repo | Não revisado; Java não roda no kernel; é framework de HAL | Avaliar: pode virar o `conexus-driver-manager` de userspace |

---

## Plano

1. **A0 + A5:** aprovar o modelo (classes 1 e 2, perfis, chaves). Decidir o destino de `driver-core/`.
2. **A2:** confirmar, no Pixel 8a, que `CONFIG_MODULES` e `vendor_dlkm` estão ativos no kernel GKI 6.1 e quais símbolos do KMI são exportados. Só leitura, sem flash.
3. **A2 + A5:** definir a cadeia de chaves de driver e o formato de `driver-manifest`.
4. **A4:** criar `framework/sdk/shared/driver-manifest.schema.json`, no mesmo padrão dos três schemas do M1.
5. **M2/M3:** implementar `conexus-driver-manager` e o primeiro driver de referência (userspace).
6. **Gate:** nenhum módulo de kernel entra em imagem `alpha` antes da revisão de segurança do A5.

---

## Referências

- `SDD.md` §1 (escopo), seção de segurança (perfil `MODULES_DISABLE=1`)
- `ADR-005` (recovery antes de flash), `ADR-007` (serviços isolados), `ADR-008` (A/B e rollback)
- `driver-core/README.md`
- Android GKI e `vendor_dlkm` (source.android.com)
- Linux `MODULE_SIG` (kernel.org, Documentation/admin-guide/module-signing)
