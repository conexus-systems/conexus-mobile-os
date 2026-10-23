"""Funções puras de segurança do instalador (sem rede, sem adb/fastboot).

Ficam separadas de install_conexus.py para serem testadas sem aparelho.
Regra geral: na dúvida, recusar (fail closed).
"""

import hashlib
import re
import zipfile
from pathlib import Path
from typing import Dict, Iterable, List

SHA256_RE = re.compile(r"^[0-9a-fA-F]{64}$")

# Imagens sem as quais não se grava nada: um pacote sem elas é recusado, não "pulado".
REQUIRED_IMAGES = ("boot.img", "vbmeta.img", "super.img")

# Limites contra zip bomb: o instalador baixa o pacote da rede.
MAX_ENTRIES = 64
MAX_TOTAL_BYTES = 16 * 1024**3


class InstallerSafetyError(Exception):
    """Entrada recusada por não passar nas checagens de segurança."""


def is_valid_sha256(value: str) -> bool:
    return bool(value) and bool(SHA256_RE.match(value))


def parse_sha256sums(text: str) -> Dict[str, str]:
    """Lê um arquivo SHA256SUMS (`<hash>  <arquivo>`) e devolve {arquivo: hash}.

    Linhas inválidas são ignoradas; hash que não tem 64 hex não entra.
    """
    sums: Dict[str, str] = {}
    for line in text.splitlines():
        parts = line.strip().split(None, 1)
        if len(parts) != 2:
            continue
        digest, name = parts
        name = name.lstrip("*").strip()
        if is_valid_sha256(digest) and name:
            sums[name] = digest.lower()
    return sums


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def verify_sha256(path: Path, expected: str) -> None:
    """Levanta InstallerSafetyError se o hash esperado for inválido ou diferente."""
    if not is_valid_sha256(expected):
        raise InstallerSafetyError("SHA-256 esperado ausente ou inválido; recusando instalar sem verificação")
    actual = sha256_file(path)
    if actual != expected.lower():
        raise InstallerSafetyError(f"SHA-256 não confere: esperado {expected.lower()}, obtido {actual}")


def safe_extract(zip_path: Path, dest: Path, max_total: int = MAX_TOTAL_BYTES) -> List[Path]:
    """Extrai só arquivos .img/.txt/.json planos, recusando caminhos suspeitos.

    Recusa: caminho absoluto, `..`, subpastas, links simbólicos, excesso de entradas
    ou de tamanho descompactado.
    """
    dest.mkdir(parents=True, exist_ok=True)
    out: List[Path] = []
    with zipfile.ZipFile(zip_path) as zf:
        infos = zf.infolist()
        if len(infos) > MAX_ENTRIES:
            raise InstallerSafetyError(f"pacote com {len(infos)} entradas (máximo {MAX_ENTRIES})")
        if sum(i.file_size for i in infos) > max_total:
            raise InstallerSafetyError("tamanho descompactado acima do limite")
        for info in infos:
            name = info.filename
            if info.is_dir():
                continue
            if name.startswith("/") or "\\" in name or ".." in Path(name).parts or "/" in name:
                raise InstallerSafetyError(f"entrada com caminho recusado: {name!r}")
            if (info.external_attr >> 16) & 0o170000 == 0o120000:
                raise InstallerSafetyError(f"link simbólico recusado: {name!r}")
            if not name.endswith((".img", ".txt", ".json")):
                raise InstallerSafetyError(f"tipo de arquivo recusado: {name!r}")
            target = dest / name
            with zf.open(info) as src, open(target, "wb") as dst:
                dst.write(src.read())
            out.append(target)
    return out


def require_images(extracted: Iterable[Path]) -> None:
    names = {p.name for p in extracted}
    missing = [n for n in REQUIRED_IMAGES if n not in names]
    if missing:
        raise InstallerSafetyError(f"imagens obrigatórias ausentes no pacote: {', '.join(missing)}")


def confirmation_phrase(serial: str, codename: str) -> str:
    """Frase que a pessoa precisa digitar para autorizar o flash deste aparelho."""
    return f"FLASH {codename} {serial}"
