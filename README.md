# slugify-filenames

tool to [slugify](https://en.wiktionary.org/wiki/slugify) strings and filenames, available as a rust library and as two standalone command-line binaries. The slugify-filenames renames all files in the given paths (or current path if none is given) making them ascii-safe and spaceless, ultimately making the task of handling filenames never require escaping. Special characters are transliterated, special chars are kept to an ascii-safe minimum and the whole filename or string are lowercased.



## installing


### command-line tools

```shell
cargo install slugify-filenames
```


### rust library

```shell
cargo add slugify-filenames
```

## usage


### command-line tools


```shell
$ slugify-string "Imagine Thís string, àscii safê and filename-sáfè"
> imagine-this-string-ascii-safe-and-filename-safe
```

```shell
$ mkdir ./tmp
$ cd ./tmp
$ echo > "Imagine not having to escape this filename.txt"
$ slugify-filenames
$ ls
imagine-not-having-to-escape-this-filename.txt
```

### rust library


```rust
use slugify_filenames::slugify_string;

let result = slugify_string("Imagine Thís string, àscii safê and filename-sáfè");

assert_eq!(result, "imagine-this-string-ascii-safe-and-filename-safe");
```
