#!/usr/bin/env python3
import pyperclip
from pathlib import Path

ROOT = Path(__file__).resolve().parent
EXCLUDE_DIRS = {ROOT / "target", ROOT / ".git"}
TEXT_EXTS = {".rs", ".json", ".toml"}

def is_text_file(path: Path) -> bool:
    return path.suffix.lower() in TEXT_EXTS or path.suffix == ""

def file_header(relpath: str) -> str:
    return f"// FILE: {relpath}\n"

def read_text(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="latin-1")
    except Exception as e:
        return f"[Ошибка чтения {path.name}: {e}]"

def main():
    parts = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        if any(ex in path.parents or path == ex for ex in EXCLUDE_DIRS):
            continue
        if not is_text_file(path):
            continue
        try:
            rel = path.relative_to(ROOT).as_posix()
        except ValueError:
            rel = str(path)
        parts.append(file_header(rel))
        parts.append(read_text(path))
        parts.append("\n")

    full = "\n".join(parts)
    pyperclip.copy(full)
    print(f"Скопировано строк: {full.count(chr(10))}")

if __name__ == "__main__":
    main()