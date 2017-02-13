Signal to Wav Converter
====

A tool for converting raw I/Q samples from the HackRF into .wav files. The tool does the down
converting necessary to ensure that the signal is audible.

```
USAGE:
    signal_to_wav [OPTIONS] <input>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --filter <freq>            Set filter frequency
    -o, --output <output>          Name of output file. (default: output.wav)
    -s, --samp_rate <samp_rate>    Set input file sample rate (default: 2e6)

ARGS:
    <input>    The path to the input file.
```