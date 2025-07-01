##
##
##
import argparse
import os, re
import yaml

import jinja2 as j

def collect_registers(root, path, collection):
    if isinstance(root, list):
        raise NotImplementedError
    elif isinstance(root, dict):
        result = {}
        if 'width' in root:
            collection.append((path, root))
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
def fixpath(path):
    elems = path.split(".")
    number = elems[-1]
    name = elems[-2]
    name = name.replace("_bits", "")

    m = re.match(pathre, number)


    return int(m.group(1)), name

def remap(path, root='registers'):
    with open(path, 'r') as file:
        yaml_content = yaml.safe_load(file)

    root = yaml_content[root]
    reg_collection = []
    bit_collection = []
    # collect full registers
    collect_registers(root, "registers", reg_collection)
    collect_bits(root, "registers", bit_collection)

    array = list(map(lambda i: dict(), range(58)))
    for name, data in bit_collection:
        reg, key = fixpath(name)
        array[reg][key] = data


    return yaml_content


def main():
    base = os.path.dirname(__file__)
    parser = argparse.ArgumentParser()

    parser.add_argument("-o", "--output", help="output file")
    parser.add_argument("-r", "--remap", help="remap")

    options = parser.parse_args()

    if options.remap:
        remap(options.remap)
        return

    path = os.path.join(base, "raspberrypi4b.jinja2")
    content = open(path).read()
    template = j.Template(content)

    if options.output:
        with open(options.output, "w") as f:
            f.write(template.render())
        return

    print(template.render())


if __name__ == '__main__':
    main()