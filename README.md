# slowcat
Prints the contents of files with a specified delay between each line.

## Why I built it
I wanted to get a feel for my benchmarks, so I wrote `for file in *.txt; do cat $file; sleep 0.2; done`. 
This only works when the files are smaller than the screen, which at some point they weren't anymore. So I want a way to go through files just slow enough so that I can spot any anomalies and error stack traces.

## Installation
```
cargo install slowcat
```

## Usage

```
# default to -n 0.2 (0.2sec per line)
slowcat -n 0.05
```

## Other Projects

<https://github.com/mreishus/slowcat> if you want to get every character out slow.
