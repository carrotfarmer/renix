# renix

renix is a blazingly-fast CLI utility for batch renaming files.

`crates.io`: [renix](https://crates.io/crates/renix)

## Installation

```bash
cargo install renix
```

## Usage

```bash
Usage: renix [OPTIONS] --path <PATH>

Options:
  -d, --path <PATH>        Path to directory containing files to be renamed
  -p, --prefix <PREFIX>    Add a prefix to the files
  -s, --suffix <SUFFIX>    Add a suffix to the files
      --no-table           Do not print table
  -r, --remove             Remove a prefix and/or suffix
  -c, --case <CASE>        Change case of file names
      --replace <REPLACE>  Replace a substring in the file names
  -e, --exclude <EXCLUDE>  Exclude certain files
  -h, --help               Print help
  -V, --version            Print version
```

## Examples

### Add a prefix to all files in a directory

```bash
renix -d /path/to/directory -p "prefix"
```

### Add a suffix to all files in a directory

```bash
renix -d /path/to/directory -s "suffix"
```

### Remove a prefix and/or suffix from all files in a directory

```bash
renix -d /path/to/directory -s "suffix" -p "prefix" -r
```

### Change case of all files in a directory

```bash
renix -d /path/to/directory -c "lower"
```

### Replace a substring in all files in a directory

```bash
renix -d /path/to/directory --replace "old,new"
```

### Exclude certain files from renaming

```bash
renix -d /path/to/directory --replace "old,new" -e /path/to/directory/file1 /path/to/directory/file2
```
