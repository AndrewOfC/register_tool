# Overview

This is a tool for examining and manipulating memory mapped registers in embedded devices via symbolic
names instead of raw hex addresses.  
The program reads a register configuration file in yaml format which contains register definitions.  

# Usage

```bash
register_tool [options] <register>[=<value>]
```



# Environment Variables:

| Var | Effect                                                               |
|-----|----------------------------------------------------------------------|
| REGISTER_TOOL_PATH    | colon separated list of directories to search for register_tool.yaml |


# Examples

## raspberrypi4b.yaml
This file was constructed with data from: [bcm2711-peripherals.pdf](https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf)

# Companion tools

The ucompleter tool can be configured with bash and zsh to provide register completions. 
Once enabled pressing TAB-TAB after register_tool will provide you with the available completions

```bash
complete -o bashdefault -o default -o nospace  -C ucompleter register_tool 
```

# Concepts

## Shadow Registers

In many cases a word in a register manages multiple functions or components.  The 'bits' section of the register file
examples splits up these functions/components for easier access.  The 'shadow' field of a register will point to the 
register that contains the entire word.  This is done so that when setting a register the existing settings of all the 
sibling bits are read first so their settings will remain unchanged.

### Example



# Layout

| Dir      | Contents                                                           |
|----------|--------------------------------------------------------------------|
| examples | examples of register files                                         |   
| python   | python helper files for rendering register files<br>(not deployed) |
| src      | Rust source code                                                   |
| target   | cargo output directory                                             |
