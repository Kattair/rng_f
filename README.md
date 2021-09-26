# rng_f
Simple program to generate a matrix and write it into a file

## Usage
rng_f row_count col_count [options]

| Option | Parameters | Default | Description |
| ------ | ---------- | ------- | ----------- |
| -d, --delimiter | delimiter: String     | " " <- space    | define the delimiter between columns |
| -o, --output    | filename: String      | "output.txt"    | specify output filename, default is output.txt |
| -r, --range     | from: i128, to: i128  | 128 bit integer | specify bottom limit and top limit for number range, which creates interval \<from, to) |
