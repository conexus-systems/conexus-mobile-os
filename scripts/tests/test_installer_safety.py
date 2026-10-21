import hashlib
import stat
import tempfile
import unittest
import zipfile
from pathlib import Path

import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from installer_safety import (  # noqa: E402
    InstallerSafetyError,
    confirmation_phrase,
    is_valid_sha256,
    parse_sha256sums,
    require_images,
    safe_extract,
    sha256_file,
    verify_sha256,
)

H = "a" * 64


def make_zip(path: Path, entries: dict, symlink: str = None):
    with zipfile.ZipFile(path, "w") as zf:
        for name, data in entries.items():
            zf.writestr(name, data)
        if symlink:
            info = zipfile.ZipInfo(symlink)
            info.external_attr = (stat.S_IFLNK | 0o777) << 16
            zf.writestr(info, "/etc/passwd")


class Sha256Tests(unittest.TestCase):
    def test_valid_and_invalid(self):
        self.assertTrue(is_valid_sha256(H))
        for bad in ("", "abc", "g" * 64, H + "0"):
            self.assertFalse(is_valid_sha256(bad), bad)

    def test_parse_sums(self):
        text = f"{H}  conexus-akita.zip\n{'B' * 64} *other.zip\nlixo\n{'c' * 10}  curto.zip\n"
        self.assertEqual(parse_sha256sums(text), {"conexus-akita.zip": H, "other.zip": "b" * 64})

    def test_verify_ok_and_mismatch(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "f.bin"
            p.write_bytes(b"conteudo")
            good = hashlib.sha256(b"conteudo").hexdigest()
            verify_sha256(p, good)
            verify_sha256(p, good.upper())
            with self.assertRaises(InstallerSafetyError):
                verify_sha256(p, H)

    def test_verify_refuses_missing_or_placeholder_hash(self):
        with tempfile.TemporaryDirectory() as d:
            p = Path(d) / "f.bin"
            p.write_bytes(b"x")
            for bad in ("", "a1b2c3", None):
                with self.assertRaises(InstallerSafetyError):
                    verify_sha256(p, bad or "")
            self.assertEqual(sha256_file(p), hashlib.sha256(b"x").hexdigest())


class ExtractTests(unittest.TestCase):
    def extract(self, entries, symlink=None, **kw):
        with tempfile.TemporaryDirectory() as d:
            z = Path(d) / "p.zip"
            make_zip(z, entries, symlink)
            return [p.name for p in safe_extract(z, Path(d) / "out", **kw)]

    def test_extracts_flat_images(self):
        self.assertEqual(sorted(self.extract({"boot.img": b"1", "super.img": b"2", "notes.txt": b"n"})),
                         ["boot.img", "notes.txt", "super.img"])

    def test_rejects_traversal_absolute_nested_backslash(self):
        for name in ("../evil.img", "/abs.img", "dir/boot.img", "a\\b.img"):
            with self.assertRaises(InstallerSafetyError, msg=name):
                self.extract({name: b"x"})

    def test_rejects_symlink(self):
        with self.assertRaises(InstallerSafetyError):
            self.extract({"boot.img": b"1"}, symlink="link.img")

    def test_rejects_unexpected_type(self):
        with self.assertRaises(InstallerSafetyError):
            self.extract({"run.sh": b"#!/bin/sh"})

    def test_rejects_too_many_entries_and_too_large(self):
        with self.assertRaises(InstallerSafetyError):
            self.extract({f"{i}.img": b"x" for i in range(65)})
        with self.assertRaises(InstallerSafetyError):
            self.extract({"boot.img": b"x" * 100}, max_total=10)

    def test_require_images(self):
        require_images([Path("boot.img"), Path("vbmeta.img"), Path("super.img")])
        with self.assertRaises(InstallerSafetyError):
            require_images([Path("boot.img"), Path("super.img")])


class ConfirmationTests(unittest.TestCase):
    def test_phrase_binds_codename_and_serial(self):
        self.assertEqual(confirmation_phrase("ABC123", "akita"), "FLASH akita ABC123")
        self.assertNotEqual(confirmation_phrase("ABC123", "akita"), confirmation_phrase("XYZ", "akita"))


if __name__ == "__main__":
    unittest.main()
