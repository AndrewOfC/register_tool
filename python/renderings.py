##
##
##
import argparse
import os

import jinja2 as j


def main():
    base = os.path.dirname(__file__)
    parser = argparse.ArgumentParser()

    parser.add_argument("-o", "--output", help="output file")

    options = parser.parse_args()

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