##
##
##
import argparse

import jinja2
import jinja2 as j
import pyclip


GPFSEL_SRC = """  
    {%for gpio_n in range(gpio_cnt)%}
    ##
    ##
    ##
    GPIO{{gpio_n}}:
        GPFSEL{{gpio_n}}:
            offset: 0x{{'%02x' % (gpio_n*4)}}
            reset: 0x0000
            read-write: "rw"
            width: 32
    GPFSEL{{gpio_n}}_bits:
    {% for i in range(9, -1, -1) %}
        FSEL{{gpio_n}}{{i}}:
            offset: 0x{{'%02x' % (gpio_n*4)}}
            reset: 0x0000
            read-write: "rw"
            bits: {{i*3+2}}:{{i*3}}
    {%- endfor %}
    {%-endfor %}
    
"""

GPSET_SRC = """
        GP{{setclr}}{{setclr_n}}:
            offset: 0x{{"%02X" % (offset+8*setclr_n)}}
            read-write: "wo"
            width: 32
        
"""

GPENBITS_SRC = """
        {% for i in range(57,-1,-1) %}
        {{setclr}}_GPIO{{"%02d" % i}}:
            offset: {%if i>31%}0x{{"%02X" % (offset+4)}}{%else%}0x{{"%02X" % offset}}{%endif%}
            bits:   {%if i>31%}{{i-32}}:{{i-32}}{%else%}{{i}}:{{i}}{%endif%}
            reset: 0x00000000
            read-write: "{{rw}}"
        {%endfor%}
"""

GP_PULLUPDOWN_SRC = """
    GPPULL_UP_DOWN_CTRL{{gpio_n}}:
        offset: 0x{{'%02x' % (gpio_n*4)}}
        reset: 0x0000
        read-write: "rw"
        width: 32
    GPULL_UP_DOWN_CTRL{{gpio_n}}_bits:
    {% for i in range(4, -1, -1) %}
        PULL_UP_DOWN_CTRL{{gpio_n}}{{i}}:
            offset: 0x{{'%02x' % (gpio_n*4)}}
            reset: 0x0000
            read-write: "rw"
            bits: {{i*2+1}}:{{i*2}}
    {%- endfor %}

"""


def main():

    parser = argparse.ArgumentParser()

    options = parser.parse_args()

    #template = j.Template(GPFSEL_SRC)
    #template = j.Template(GPSET_SRC)
    #regs = template.render(offset=0x1C)

    template = j.Template(GPSET_SRC)
    regs = template.render(setclr_n=0, setclr="set", offset=0x1c)
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="set", offset=0x1c)
    print(regs)

    template = j.Template(GPSET_SRC)
    regs = template.render(setclr_n=1, setclr="clr", offset=0x1c, rw="wo")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="clr", offset=0x28, rw="wo")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="lvl", offset=0x34, rw="ro")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="ed", offset=0x40, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="ren", offset=0x4C, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="fen", offset=0x58, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="hen", offset=0x64, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="len", offset=0x70, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="aren", offset=0x7C, rw="w1c")
    print(regs)

    template = j.Template(GPENBITS_SRC)
    regs = template.render(setclr="afen", offset=0x88, rw="w1c")
    print(regs)

    for i in range(4):
        template = j.Template(GP_PULLUPDOWN_SRC)
        regs = template.render(gpio_n=i, offset=0xe4)
        print(regs)

    return


if __name__ == '__main__':
    main()