import json
from pathlib import Path
import tempfile
import unittest

from generate_update_manifest import manifest


class UpdateManifestTests(unittest.TestCase):
    def test_feed_contains_the_installer_signature_and_exact_release_url(self):
        with tempfile.TemporaryDirectory() as directory:
            installer = Path(directory) / "SpeakFlow_1.0.2_x64-setup.exe"
            installer.write_bytes(b"installer fixture")
            signature = Path(str(installer) + ".sig")
            with self.assertRaises(FileNotFoundError):
                manifest("1.0.2", "1.0.2", installer, "Notes")
            signature.write_text("signed-installer\n", encoding="utf-8")
            feed = json.loads(json.dumps(manifest("1.0.2", "1.0.2", installer, "Notes")))
            platform = feed["platforms"]["windows-x86_64"]
            self.assertEqual(feed["version"], "1.0.2")
            self.assertEqual(platform["signature"], "signed-installer")
            self.assertEqual(platform["url"], "https://github.com/khentmoba/SpeakFlow/releases/download/1.0.2/SpeakFlow_1.0.2_x64-setup.exe")
            with self.assertRaises(ValueError):
                manifest("1.0.3", "1.0.3", installer, "")
            signature.write_text("", encoding="utf-8")
            with self.assertRaises(ValueError):
                manifest("1.0.2", "1.0.2", installer, "")


if __name__ == "__main__":
    unittest.main()
