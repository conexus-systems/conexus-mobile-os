"""Valida as fixtures contra os JSON Schemas: valid/ deve passar, invalid/ deve falhar.

As mesmas fixtures são lidas pelo teste do crate Rust (framework/sdk/rust/tests/fixtures.rs),
o que mantém schema e Rust com o mesmo comportamento.
Uso: python3 framework/sdk/tests_schema_fixtures.py   (requer o pacote jsonschema)
"""
import json
import sys
from pathlib import Path

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parent
SCHEMAS = {
    "tile": ROOT / "shared/tile-manifest.schema.json",
    "app": ROOT / "shared/app-manifest.schema.json",
    "notification": ROOT / "shared/notification.schema.json",
}
validators = {}
for kind, path in SCHEMAS.items():
    schema = json.loads(path.read_text())
    Draft202012Validator.check_schema(schema)
    validators[kind] = Draft202012Validator(schema)

failures = []
count = 0
for expect_valid, folder in ((True, "valid"), (False, "invalid")):
    for f in sorted((ROOT / "fixtures" / folder).glob("*.json")):
        kind = f.name.split("-")[0]
        ok = validators[kind].is_valid(json.loads(f.read_text()))
        count += 1
        if ok != expect_valid:
            failures.append(f"{folder}/{f.name}: esperado {'válido' if expect_valid else 'inválido'}")
print(f"{count} fixtures, {len(failures)} divergências")
for line in failures:
    print(" -", line)
sys.exit(1 if failures else 0)
