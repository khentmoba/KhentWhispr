import ast
import json
import re
import tomllib
import unittest
from pathlib import Path


TAURI_DIR = Path(__file__).resolve().parent.parent
RESOURCES_DIR = TAURI_DIR / "resources"


def _contains_cjk(text):
    return any(
        "\u3000" <= char <= "\u303f"
        or "\u3400" <= char <= "\u9fff"
        or "\uff00" <= char <= "\uffef"
        for char in text
    )


def _runtime_string_literals(source):
    tree = ast.parse(source)
    docstrings = set()
    for node in ast.walk(tree):
        if isinstance(
            node, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)
        ) and node.body:
            first = node.body[0]
            if (
                isinstance(first, ast.Expr)
                and isinstance(first.value, ast.Constant)
                and isinstance(first.value.value, str)
            ):
                docstrings.add(first.value)
    return [
        node.value
        for node in ast.walk(tree)
        if isinstance(node, ast.Constant)
        and isinstance(node.value, str)
        and node not in docstrings
    ]


def _json_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for key, item in value.items():
            yield from _json_strings(key)
            yield from _json_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from _json_strings(item)


class EnglishRuntimeStringsTests(unittest.TestCase):
    def test_production_python_runtime_literals_are_english(self):
        for path in RESOURCES_DIR.glob("*.py"):
            if path.name.startswith("test_"):
                continue
            source = path.read_text(encoding="utf-8")
            for value in _runtime_string_literals(source):
                self.assertFalse(_contains_cjk(value), f"{path.name}: {value!r}")

    def test_build_rust_string_literals_are_english(self):
        for name in ("build.rs", "build_support.rs"):
            source = (TAURI_DIR / name).read_text(encoding="utf-8")
            literals = re.findall(r'"(?:\\.|[^"\\])*"', source)
            for literal in literals:
                self.assertFalse(_contains_cjk(literal), f"{name}: {literal}")

    def test_tauri_config_and_capability_strings_are_english(self):
        paths = [
            TAURI_DIR / "tauri.conf.json",
            *(TAURI_DIR / "capabilities").glob("*.json"),
        ]
        for path in paths:
            values = _json_strings(json.loads(path.read_text(encoding="utf-8")))
            for value in values:
                self.assertFalse(_contains_cjk(value), f"{path.name}: {value!r}")

    def test_cargo_package_description_is_english(self):
        manifest = tomllib.loads(
            (TAURI_DIR / "Cargo.toml").read_text(encoding="utf-8")
        )
        self.assertFalse(_contains_cjk(manifest["package"]["description"]))


if __name__ == "__main__":
    unittest.main()
