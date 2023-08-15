# transfer.sh-cli
A rust client for [transfer.sh](https://transfer.sh/)

**Help**
```
$ transfer --help
Unofficial Transfer.sh CLI

Usage: transfer.exe [OPTIONS]

Options:
  -h, --help                             Print help
  -dow, --download <link> <file name>    Dowload file
  -del, --delete <delete-link>           Delete file

```

**Usage sample:**
```
# Normal Upload
$ transfer /path/to/file

# Delete your file
$ transfer -del <delete-link>

# Download file
$ transfer -dow <link> <file name>
```
