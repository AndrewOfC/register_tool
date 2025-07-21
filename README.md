# Overview

This is a tool for examining and manipulating memory mapped registers in embedded devices via symbolic
names instead of raw addresses.  
The program reads a register configuration file in yaml format which contains register definitions.  

# Usage

```bash
register_tool [options] <path>[=<value>]...
```

# Building

## Arm64

```bash
cargo build --target aarch64 --bin register_tool
```



## Parameters and options

| P/O       | meaning                                                       |
|-----------|---------------------------------------------------------------|
| path      | path to register definition                                   |
| value     | hex, octal or binary value to set register to                 |
| -d        | Dump the register definition, do not set or read              |
| -f <file> | Override register file(s) that might be in REGISTER_TOOL_PATH |
| -t        | Test mode.  Do not map memory, allocate a black of 'length'    |


# Concepts

## Path

A 'path' describes the location of a register in the yaml definition file.  The notion is similar
to dereferencing a python or javascript object.  A dot(.) will access fields in an associative array
or hash block and [] may be used to access individual array members.

### Example

```yaml
GPIO:
    pins:
         # 0 
         - set:
              offset: 0x1C
           clear:
              offset: 0x28
         # 1     
         - set:
              offset: 0x1C
           clear:
              offset: 0x28
```

__GPIO.pins[0]__

will access the pin 0

# Environment Variables:

| Var | Effect                                                               |
|-----|----------------------------------------------------------------------|
| REGISTER_TOOL_PATH    | colon separated list of directories to search for register_tool.yaml |

# How To

## Define a device

The root of your yaml configuration file should contain the following fields

```yaml
base: 0x7E200000
length: 0x2000
device: "/dev/gpiomem"
```

| field  | value                                                  |
|--------|--------------------------------------------------------|
| base   | base addrewss to apply with mmap                       |
| length | length to apply to mmap                                |
| device | path to device to open for mmap.  Defaults to /dev/mem |


## Define a register
Registers may be defined in any hierarchy that makes sense for your project.  They can also be duplicated and aliased
for convenience.

```yaml
completion-metadata:
  root: "registers"
  terminus: ["offset", "bits"]

base: 0x0000 # base address
length: 0x2000

registers:
  GPIO:
     
      register:
          offset: 0x00 
          read-write:  rw|ro|wo|w1c
          description: general purpose input/output

      pin:
          bits: hi:lo # inclusive
          parent: "GPIO.register"          
```

## completion-metadata
This is information that the [ucompleter](https://github.com/AndrewOfC/ucompleter) tool will use to provide completions of your registers
on the bash command line. It is not required, but it is recommended.

| Field | Purpose                                                        |
|-------|----------------------------------------------------------------|
| root  | Path to the element where register definitions are to be found |
 |terminus| If any of these fields are present in hash as the tree is descended the descent is stopped| 


## Register Definition
| Field       | Purpose                                                                                                             |
|-------------|---------------------------------------------------------------------------------------------------------------------|
| offset      | offset from memory base                                                                                             |
| bits        | hibit:lobit selection of individual bits in a word(inclusive)<br> 31:31 first bit in register<br>1:0 last two bits  |
| read-write  | rw: read-write<br>ro: read-only<br>wo: write only<br>w1c: write-once-to-clear                                       |
| description | Description of register                                                                                             |
| parent      | If a required field is not found in block, parent will be checked(recursive). This is a path from the defined root. |



## Provide completions
Command line completions can be a major convenience

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

### Example

# Layout

| Dir      | Contents                                                           |
|----------|--------------------------------------------------------------------|
| examples | examples of register files                                         |   
| python   | python helper files for rendering register files<br>(not deployed) |
| src      | Rust source code                                                   |
| target   | cargo output directory                                             |
