#
# SPDX-License-Identifier: MIT
#
# Copyright (c) 2025 Andrew Ellis Page
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#
import argparse
import os
from io import StringIO

class Formatter(object):
    def __init__(self):
        return

    def format(self, text):
        raise NotImplemented()
    
    def license_present(self, path):
        ident = "SPDX-License-Identifier"
        
        with open(path, "r") as f:
            for line in f:
                if ident in line:
                    return True

        return False

class SharpFormatter(Formatter):
    def __init__(self):
        return

    def format(self, text):
        istream = StringIO(text)
        ostream = StringIO()
        for line in istream:
            print(f"# {line}", end="", file=ostream)
        return ostream.getvalue()

class RustFormatter(Formatter):
    def __init__(self):
        return

    def format(self, text):
        istream = StringIO(text)
        ostream = StringIO()
        for line in istream:
            print(f"// {line}", end="", file=ostream)
        return ostream.getvalue()


class Jinja2Formatter(Formatter):
    def __init__(self):
        return
    
    def format(self, text):
        ostream = StringIO()
        print("{#-", file=ostream)
        print(text, file=ostream)
        print("-#}", file=ostream)
        return ostream.getvalue()

class CFormatter(Formatter):
    def __init__(self):
        return

    def format(self, text):
        istream = StringIO(text)
        ostream = StringIO()

        print("/*", file=ostrealicense_annotator.pym)
        for line in istream:
            print(f" * {line}", end="", file=ostream)
        print(" */", file=ostream)
        return ostream.getvalue()




class LicenseAnnotator(object):
    C_FORMATTER = CFormatter()
    SHARP_FORMATTER = SharpFormatter()

    SUFFIX_FORMATTERS = {
        ".py": SHARP_FORMATTER,
        ".sh": SHARP_FORMATTER,
        ".toml": SHARP_FORMATTER,
        ".yaml": SHARP_FORMATTER,
        ".yml": SHARP_FORMATTER,
        ".jinja2": Jinja2Formatter(),
        ".rs": RustFormatter(),
        ".c": C_FORMATTER,
        ".h": C_FORMATTER,
        ".cpp": C_FORMATTER,
        ".hpp": C_FORMATTER,
        ".cxx": C_FORMATTER,
        ".hxx": C_FORMATTER,
    }
    def __init__(self, source, ignore_names=None, verbose=False, dry_run=False):
        if ignore_names is None:
            ignore_names = [".git", ".venv", ".env", "env", "venv"]
        self._verbose = verbose
        self._dry_run = dry_run
        self._ignore_names = set(ignore_names)
        self._license_cache = {}
        self._source = open(source, "r").read()
        return

    def apply_license(self, filepath, ext):

        formatter = self.SUFFIX_FORMATTERS[ext]
        if ext in self._license_cache:
            license_text = self._license_cache[ext]
        else:
            self._license_cache[ext] = license_text = formatter.format(self._source)

        if formatter.license_present(filepath):
            return

        with open(filepath, "r") as f:
            text = f.read()
        with open(filepath, "w") as f:
            f.write(license_text)
            f.write(text)

    def _ignore(self, dirpath, path):

        while os.path.commonpath([path, dirpath]) == dirpath:
            if os.path.basename(path) in self._ignore_names:
                return True
            path = os.path.dirname(path)
        return False

    def apply(self, dirpath):
        for root, dirs, files in os.walk(dirpath):

            for file in files:

                if self._ignore(dirpath, root):
                    break

                filepath = os.path.join(root, file)
                _, ext = os.path.splitext(filepath)

                if ext not in self.SUFFIX_FORMATTERS:
                    continue
                formatter = self.SUFFIX_FORMATTERS[ext]
                if not formatter.license_present(filepath):
                    if self._verbose:
                        print(f"# Applying license to {filepath}")
                    self.apply_license(filepath, ext)
                else:
                    if self._verbose:
                        print(f"# Skipping {filepath} as license is already present")

        return


DESCRIPTION = """General purpose tool for annotating software licenses to multiple files."""

def main():
    basedir = os.path.dirname(__file__)
    parser = argparse.ArgumentParser(description=DESCRIPTION)

    parser.add_argument("-f", "--file", default=os.path.join(basedir, "..", "LICENSE"),
                        help="license source")
    parser.add_argument("-d", "--dir", default=os.path.join(basedir, ".."), help="directory to annotate")
    parser.add_argument("-v", "--verbose", action="store_true", help="verbose output")
    parser.add_argument("--dry-run", action="store_true", help="dry run")

    options = parser.parse_args()

    LicenseAnnotator(options.file, verbose=options.verbose).apply(options.dir)


    return

if __name__ == '__main__':
    main()