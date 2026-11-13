import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

MODULE_PATH = Path(__file__).parents[1] / "src" / "bloxpm.py"
spec = importlib.util.spec_from_file_location("bloxpm", MODULE_PATH)
bloxpm = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bloxpm)

class BloxPMTests(unittest.TestCase):
    def test_create_verify_and_install(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            source = root / "source"
            (source / "bin").mkdir(parents=True)
            (source / "blox.json").write_text(json.dumps({"id":"test.app", "name":"Test App", "version":"1.0"}), encoding="utf-8")
            (source / "bin" / "app.txt").write_text("payload", encoding="utf-8")
            package = root / "test.blox"
            bloxpm.create(source, package)
            manifest = bloxpm.verify(package)
            self.assertEqual(manifest["id"], "test.app")
            destination = root / "apps"
            bloxpm.install(package, destination)
            self.assertEqual((destination / "test.app" / "bin" / "app.txt").read_text(), "payload")

    def test_rejects_path_traversal(self):
        with self.assertRaises(ValueError):
            bloxpm.safe_member("../outside.txt")

if __name__ == "__main__":
    unittest.main()
