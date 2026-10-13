import unittest
import tempfile
import shutil
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import validate_docs

class TestValidateDocs(unittest.TestCase):
    def setUp(self):
        self.test_dir = Path(tempfile.mkdtemp())

    def tearDown(self):
        shutil.rmtree(self.test_dir)

    def test_required_files_check(self):
        # Missing docs should produce errors
        errors = validate_docs.validate_required_files(self.test_dir)
        self.assertTrue(len(errors) > 0)

        # Create dummy required files
        for req in validate_docs.REQUIRED_DOCS:
            p = self.test_dir / req
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text("# Dummy", encoding="utf-8")

        errors = validate_docs.validate_required_files(self.test_dir)
        self.assertEqual(len(errors), 0)

if __name__ == "__main__":
    unittest.main()
