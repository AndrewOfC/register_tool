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
import re
import sys
import argparse
from dataclasses import Field
from operator import attrgetter

import lxml.etree as etree

class Extractor(object):

    bitfieldRE = re.compile(r"^([0-9]+):([0-9]+)$")
    @classmethod
    def bitFieldExtract(clazz, s:str):
        """ [hi:lo] inclusive  """

        m = clazz.bitfieldRE.match(s)
        if m is None:
            raise ValueError(f"Invalid bitfield specification {s}")
        hi = int(m.group(1),0)
        lo = int(m.group(2),0)
        if hi < lo:
            raise ValueError(f"Invalid bitfield specification {s}")

        return (hi, lo)

    converters = { 'addressOffset': lambda x: int(x, 0),
                   'bitRange': lambda x: Extractor.bitFieldExtract(x),
                   'size': lambda x: int(x, 0),
                   'resetValue': lambda x: int(x, 0),
                   }
    defaultConverter = (lambda x: x)

    def __init__(self, tree: etree._Element, fields=None):
        if fields is None:
            fields = map(lambda x: x.tag, tree.getchildren())

        for field in fields:
            node = tree.find(field)
            converter = self.converters.get(node.tag, self.__class__.defaultConverter)
            setattr(self, '_' + node.tag, converter(node.text))
            prop = property(lambda self: attrgetter(self, '_' + node.tag))
            setattr(self.__class__, node.tag, prop)
        return

class Field(object):
    def __init__(self, tree: etree._Element):
        return

    name = property(lambda self: self._name)

class Register(Extractor):
    def __init__(self, tree: etree._Element):
        super().__init__(tree, ("name", "access", "desscription", "addressOffset"))
        # self._name = tree.xpath("./name/text()")[0]
        # self._access = tree.xpath("./access/text()")[0]
        # self._description = tree.xpath("./description/text()")[0]
        # self._offset = int(tree.xpath("./addressOffset/text()")[0], 0)

        for field in tree.xpath("./fields/field"):
            f = Field(field)


        return

    name = property(lambda self: self._name)

class Peripheral(object):
    def __init__(self, tree: etree._Element):

        self._registers = {}

        self._name = tree.xpath("./name/text()")[0]
        self._description = tree.xpath("./description/text()")[0]
        self._baseAddress = int(tree.xpath("./baseAddress/text()")[0], 0)

        for reg in tree.xpath("./registers/register"):
            reg = Register(reg)
            self._registers[reg.name] = reg

        return

    name = property(lambda self: self._name)


class SVDConverter(object):
    def __init__(self):
        self._basePeripherals = {}

        return

    def reset(self):
        self._basePeripherals = {}

    def convert(self, svd, outf):

        doc = etree.parse(svd)
        root = doc.getroot()
        # peripherals

        peripherals = root.xpath("./peripherals/peripheral[not(@derivedFrom)]")
        
        for peripheral in peripherals:
            p = Peripheral(peripheral)
            self._basePeripherals[p.name] = p

        return

def main():
    parser = argparse.ArgumentParser(description='Convert CMSIS SVD to YAML')

    parser.add_argument('svd', help='CMSIS SVD file')

    options = parser.parse_args()

    converter = SVDConverter()

    converter.convert(options.svd, sys.stdout)


    return

if __name__ == '__main__':

    main()