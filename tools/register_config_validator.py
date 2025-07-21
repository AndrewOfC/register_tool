#! /usr/bin/env python

import argparse
import re
import sys

import yaml

RE = re.compile(r"([^.\[\]\\]+)(?:\.)?|(?:\[(\d+)]?)?")


def yaml_descend(y, path):
    current = y
    for key, index in RE.findall(path):
        if not key and not index:
            continue
        if key:
            if key in current:
                current = current[key]
                continue
            return None, f"key {key} not found in {path}"

        index = int(index) if index else 0
        if index >= len(current):
            return None, f"index {index} out of range in {path}"
        current = current[index]

    return current, ''

class Validator(object):

    read_write_tags = {"rw", "ro", "wo", "w1c" }

    def __init__(self, verbose=False, warnings_as_errors=False):
        self.verbose = verbose
        self.warnings_as_errors = warnings_as_errors
        self.reset()
        return

    def reset(self):
        self.errors = []
        self.warnings = []
        self.count = 0


    def get_field_or_parent(self, doc, path, reg, field):

        if field in reg:
            return reg[field], None
        if 'parent' not in reg:
            return None, f"{field} not found for path: {path}"
        parent, error = yaml_descend(doc, reg['parent'])
        if not parent:
            return None, f"{field} not found for path: {error}"
        return self.get_field_or_parent(doc, path, parent, field)

    def check_register(self, doc, path, reg):
        ##
        ## validate parent
        ##
        self.count += 1
        parent_path = None
        parent_reg = None
        if 'parent' in reg:
            parent_path = reg['parent']
            parent_reg, error = yaml_descend(doc, parent_path)
            if error:
                self.errors.append(f"parent {parent_path} not found for {path}")
        ##
        ## Validate shadow
        ##
        shadow_path, error = self.get_field_or_parent(doc, path, reg, 'shadow')
        if shadow_path:
            shadow_reg, error = yaml_descend(doc, shadow_path)
            if error:
                self.errors.append(f"shadow '{shadow_path}' not found for {path}")
        ##
        ## offset
        ##
        offset, error = self.get_field_or_parent(doc, path, reg, 'offset')
        if offset is None:
            self.errors.append(f"offset not specified for {path}")
        elif not isinstance(offset, int):
            self.errors.append(f"invalid offset '{offset}' for {path}")

        ##
        ## Validate read-write
        ##h
        read_write, error = self.get_field_or_parent(doc, path, reg, 'read-write')
        if not read_write:
            self.warnings.append(f"read-write not specified for {path}")
        else:
            if read_write not in self.read_write_tags:
                self.errors.append(f"invalid read-write '{read_write}' for {path}")
        ##
        ## width or bits
        ##
        width, error = self.get_field_or_parent(doc, path, reg, 'width')
        if (width is not None) and (not isinstance(width, int)):
            self.errors.append(f"invalid width specification '{width}' for {path}")
            width = None
        bits, error = self.get_field_or_parent(doc, path, reg, 'bits')
        if not width and not bits:
            self.warnings.append(f"width or bits not specified for {path}")
        ##
        ## check bits
        ##
        if bits is not None and not isinstance(bits, str):
            self.errors.append(f"invalid bits specification '{bits}' for {path}")
        elif bits:
            elems = bits.split(":")
            if len(elems) != 2:
                self.errors.append(f"invalid bits specification '{reg['bits']}' for {path}")
            try:
                start = int(elems[0])
                end = int(elems[1])
                if start < end:
                    self.errors.append(f"invalid bits specification '{reg['bits']}' for {path}")
                if start < 0 or end > 31:
                    self.errors.append(f"invalid bits specification '{reg['bits']}' for {path}")
            except ValueError:
                self.errors.append(f"invalid bits specification '{reg['bits']}' for {path}")

        return

    RegFields = {"offset", "bits", "read-write", "shadow"}
    def findAndCheckRegisters(self, doc, path, root):

        if isinstance(root, dict):
            for field in self.RegFields: # todo set intersection
                if field in root: # register
                    if self.verbose:
                        print(f"# checking {path}")
                    self.check_register(doc, path, root)
                    return

            for key, value in root.items():
                sep = "." if path else ""
                self.findAndCheckRegisters(doc, path + sep + key, value)

        if isinstance(root, list):
            for index, value in enumerate(root):
                self.findAndCheckRegisters(doc, path + f"[{index}]", value)

    def validate(self, path:str, root_key=None):
        with open(path, "r") as f:
            doc = yaml.load(f, Loader=yaml.CLoader)

        errors = []
        root = doc
        if root_key is not None:
            root = doc[root_key]
        elif 'completion-metadata' in doc:
            rootpath, error = yaml_descend(doc, 'completion-metadata.root')
            if error:
                errors.append(error)
            else:
                if rootpath not in doc:
                    errors.append(f"root {rootpath} not found in {path}")
                else:
                    root = doc[rootpath]

        self.findAndCheckRegisters(root, "", root)
        return (self.errors, self.warnings, self.count)


def main():

    parser = argparse.ArgumentParser()

    parser.add_argument("file", nargs='*', help="file to validate")
    parser.add_argument("-v", "--verbose", action="store_true", help="verbose output")
    parser.add_argument("-W", "--warnings-as-errors", action="store_true", help="treat warnings as errors")
    parser.add_argument("-q", "--quiet", action="store_true", help="quiet output")

    options = parser.parse_args()

    for path in options.file:
        errors, warnings, count = Validator(verbose=options.verbose,
                  warnings_as_errors=options.warnings_as_errors).validate(path)

    if not options.quiet:
        if warnings:
            print(f"warnings: {len(warnings)}")
            for warning in warnings:
                print(f"  {warning}")
        if errors:
            print(f"errors: {len(errors)}")
            for error in errors:
                print(f"  {error}")
    if errors:
        sys.exit(1)
    if warnings and options.warnings_as_errors:
        sys.exit(1)
    sys.exit(0)

if __name__ == '__main__':
    main()