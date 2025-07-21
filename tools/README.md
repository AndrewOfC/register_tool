
# Overview

Tools for deployment

# Contents

| Entry                                | Use                                                                        |
|--------------------------------------|----------------------------------------------------------------------------|
| [ucompleter](ucompleter) (submodule) | Rust program that can provide bash/zsh(?) completions based on config file |
| [register_config_validator.py](register_config_validator.py)     | Python script to validate register configurations                          |

# register_config_validator.py

Checks the validity of a register file.  Will gather all warnings and errors from valid yaml formatted file before exiting

## Requirements

[PyYaml](https://pypi.org/project/PyYAML/)

```bash
pip install pyyaml
```

## Usage

With this file:
```yaml
---
completion-metadata:
  root: registers

registers:
  GPIO:
    pins:
      - bits: "1:1"
        parent: GPIO.words.pin1

    words:
      pin0:
        offset: 0
        read-write: rw
        width: 32
```
you would get the following output and exit status of 1:
```text
warnings: 1
  read-write not specified for GPIO.pins[0]
errors: 2
  parent GPIO.words.pin1 not found for GPIO.pins[0]
  offset not specified for GPIO.pins[0]

```

The first error is due to the 'parent' for pins[0] is set as words.pin1 
(a common typo) and so the shadow is invalid.  As a result, we get the other
error because without the parent's data no offset is specified.

If you change GPIO.words.pin1 to GPIO.words.pin0 the validator will run without
warnings or errors and exits with status 0.