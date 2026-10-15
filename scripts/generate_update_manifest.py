"""Generate Tauri's Windows update feed from a signed release installer."""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import re
from urllib.parse import quote


def manifest(version: str, tag: str, installer: Path, notes: str) -> dict:
    if not re.fullmatch(r"\d+\.\d+\.\d+", version):
        raise ValueError("Expected an X.Y.Z release version")
    if installer.name != f"SpeakFlow_{version}_x64-setup.exe" or not installer.is_file():
        raise ValueError("Installer does not match the release version")
    signature = Path(str(installer) + ".sig").read_text(encoding="utf-8").strip()
    if not signature:
        raise ValueError("Missing updater signature")
    return {
        "version": version,
        "notes": notes,
        "pub_date": datetime.now(timezone.utc).isoformat(),
        "platforms": {
            "windows-x86_64": {
                "url": f"https://github.com/khentmoba/KhentWhispr/releases/download/{quote(tag, safe='')}/{installer.name}",
                "signature": signature,
            },
        },
    }


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("version")
    parser.add_argument("tag")
    parser.add_argument("installer", type=Path)
    parser.add_argument("--notes", default="")
    args = parser.parse_args()
    output = args.installer.parent / "latest.json"
    output.write_text(json.dumps(manifest(args.version, args.tag, args.installer, args.notes), indent=2) + "\n", encoding="utf-8")
    print(f"Updater manifest: {output}")


if __name__ == "__main__":
    main()
