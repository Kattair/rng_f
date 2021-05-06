# rng_f
Simple program to generate a matrix and write it into a file

## Usage
rng_f rows_count cols_count [options]

| Option | Parameters | Description |
| ------ | ---------- | ----------- |
| -s, --spaces | | generate one space between columns for better readibility or parsing |
| -o, --output | filename | specify output filename, default is output.txt |
| -r, --range | from to | specify bottom limit and top limit for number range, which creates interval \<from, to) |

## TODO
Well, it works, but the logic is basically if-then clownfiesta.
I would like to:
- [ ] create better system for parsing command line arguments
- [ ] create more generic way of specifying action for specific command line arguments
 - create generics for command line flags? (flag with or without additional parameters)
     - --spaces is simple boolean flag
     - --output needs one argument
     - --range needs two arguments
- [ ] previous tasks might hopefully alter those two for cycles in main.rs
