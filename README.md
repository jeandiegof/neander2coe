## Neander COE Generator 0.1

Generates a Xilinx COE file from a Neander program.

```
USAGE:
    neander-coe --input <FILE> --output <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     Neander program.
    -o, --output <FILE>    Resulting COE file.
```

### Example

`example.asm`:

```
NOP
NOP
LDA 30
STA 06
```

results in:

```
memory_initialization_radix=16;
memory_initialization_vector=
00,
00,
20,
30,
10,
06,
```
