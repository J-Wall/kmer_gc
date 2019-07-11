# kmer_gc
Generates a 2d histogram of GC content vs k-mer counts

```
USAGE:
    kmer_gc [OPTIONS] <INPUT> -k <K>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -k <K>
    -n, --nbins <N>          Number of bins in k-mer dimension (memory use will be O(NK), where K is k-mer length)
                             [default: 1000]
    -s, --sep <SEPARATOR>    Separator for output [default:  ]

ARGS:
    <INPUT>    Input file (with k-mer in 1st column and counts in 2nd column)
```
