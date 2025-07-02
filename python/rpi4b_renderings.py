##
##
##
import argparse
import os, re
import sys

import yaml

import jinja2 as j

def collect_registers(root, path, collection):
    if isinstance(root, list):
        raise NotImplementedError
    elif isinstance(root, dict):
        result = {}
        if 'width' in root:
            collection.append([path, root])
        for key, value in root.items():
            collect_registers(value, path + "." + key, collection)
        return result
    else:
        return root


def collect_bits(root, path, collection):
    if isinstance(root, list):
        raise NotImplementedError
    elif isinstance(root, dict):
        if 'bits' in root:
            collection.append((path, root))
        for key, value in root.items():
            collect_bits(value, path + "." + key, collection)

pathre = re.compile(r"gp(\d+)")
def fix_bit_path(path):
    elems = path.split(".")
    number = elems[-1]
    name = elems[-2]
    name = name.replace("_bits", "")

    m = re.match(pathre, number)

    return int(m.group(1)), name

TEMPLATE_SRC="""---
{{ preamble }}
registers:
    GPIO:
        pins:
            {% for reg in register_bits -%}
            ##
            ## {{loop.index0}}
            ##
            - {% for key, value in reg.items() -%}
              {{ key }}: {% for key2, value2 in value.items() %}
                  {{ key2 }}: {{ formatter(key2, value2) }}
                  {%- endfor %}
              {% endfor %}
            {%endfor%}
        words:
            {%- for key,value in register_words %}
            {{ key }}:
                {%- for key2, value2 in value.items() %}
                {{ key2 }}: {{ formatter(key2, value2) }}
                {%- endfor -%}
            {% endfor -%}
"""

def quotify(value):
    return f"\"{value}\""

FORMATTERS = {
    "description": quotify,
    "read-write": quotify,
    "bits": quotify,
    "offset": (lambda value: f"0x{value:02X}" ),

}
def format_value(key, value):
    func = FORMATTERS.get(key, lambda value: value)
    return func(value)

def remap(path, preamble, outf, root='registers'):
    with open(path, 'r') as file:
        yaml_content = yaml.safe_load(file)

    root = yaml_content[root]
    reg_collection = []
    bit_collection = []
    # collect full registers
    collect_registers(root, "registers", reg_collection)
    collect_bits(root, "registers", bit_collection)

    bits_array = list(map(lambda i: dict(), range(58)))
    for name, data in bit_collection:
        reg, key = fix_bit_path(name)
        bits_array[reg][key] = data

    for entry in reg_collection:
        elems = entry[0].split(".")
        entry[0] = elems[-1]

    template = j.Template(TEMPLATE_SRC)
    outf.write(template.render(register_bits=bits_array,
                          register_words=reg_collection,
                          formatter=format_value,
                               preamble=preamble))

def main():
    base = os.path.dirname(__file__)
    parser = argparse.ArgumentParser()

    parser.add_argument("-o", "--output", help="output file")
    parser.add_argument("-r", "--remap", help="remap")

    options = parser.parse_args()

    preamble = open(os.path.join(base, "rpi4b_preamble.txt")).read()

    if options.output:
        outf = open(options.output, 'w')
    else:
        outf = sys.stdout

    if options.remap:
        remap(options.remap, preamble, outf)
        return

    path = os.path.join(base, "raspberrypi4b.jinja2")
    content = open(path).read()
    template = j.Template(content)

    outf.write(template.render(preamble=preamble))


if __name__ == '__main__':
    main()