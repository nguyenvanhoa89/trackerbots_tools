IQ Converter
====

A simple tool for converting the raw I/Q samples from the HackRF into formats that can be used by
other programs.

It can also be used as a library, returning an iterator over tuples of 32-bit floating point values.

```
USAGE:
    converter.exe [OPTIONS] <input> <output>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --encoding <encoding>    The encoding to use for the output file. [default: LittleEndian]
                                 [values: LittleEndian, BigEndian, Text]

ARGS:
    <input>     The name of the input file
    <output>    The name of the output file
```
