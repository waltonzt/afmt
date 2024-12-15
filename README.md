# 🚀 A Fast Configrable Salesforce Apex Formatter

`afmt` is written in Rust🦀, uses [tree-sitter sfapex parser](https://github.com/aheber/tree-sitter-sfapex).

Note. it's in its early phase, don't expect to use it in production code yet.

# ✨ v.s. Prettier Apex

Both afmt and [Prettier Apex](https://github.com/dangmai/prettier-plugin-apex)
provide clear formatted Apex code, leveraging the same line wrapping algorithm:
[Wadler's
Pretty-Print](https://homepages.inf.ed.ac.uk/wadler/papers/prettier/prettier.pdf).

However, unlike Prettier's "opinionated" approach, afmt strives to provide a
flexible and configurable user experience. As a result, the formatting outcomes
of the two tools vary, reflecting their fundamentally different guiding
principles.
<br>

## Other highlights:

| Feature                   | afmt                                      | Prettier Apex                             |
|---------------------------|-------------------------------------------|-------------------------------------------|
| **Maturity**              | Brand new | Battle tested for years|
| **Dependencies**       | N/A (a standalone binary that works independently) | NodeJS + prettier package|
| **Performance**            |Fast (Rust + C) |Relatively slower (NodeJS + Java jar)|
| **Parser**            |sfapex (open source) |Jorje (close source)|
| **Open Source**           | Yes| Yes|

<br>

# 📟 Progress

| Feature                                         | Progress       |
| ----------------------------------------------- | -------------- |
| Recognize Apex nodes                            | ████████████ 100%  |
| Support `.afmt.toml` for configuration          | ████████████ 100%         |
| Proper indentation                              | ████████████ 100%  |
| Support SOQL                                    | ████████████ 100%  |
| Support SOSL                                    | ████████████ 100%  |
| Line wrapping               | ████████████ 100%  |
| Support comment (line comment and block comment)| █░░░░░░░░░ 10%  |

<br>

# 🔧 How to use

Download the binary from the [release page](https://github.com/xixiaofinland/afmt/releases/latest). It
supports Linux, MacOS, and Windows.

Extract and run `afmt -h` to check the supported parameters.

```
Apex format tool (afmt): v0.0.19

Usage: afmt [OPTIONS] <FILE>

Arguments:
  <FILE>  The relative path to the file to parse

Options:
  -c, --config <CONFIG>  Path to the .afmt.toml configuration file
  -w, --write            Write the formatted result back to the file
  -h, --help             Print help
  -V, --version          Print version

EXAMPLES:

# Dry run: print the result without overwriting the file
afmt ./file.cls

# Format and write changes back to the file
afmt --write src/file.cls

# Use a specific config file
afmt --config .afmt.toml ./file.cls
```

## Simple use scenarios:

- create a `file.cls` file next to binary with valid Apex format
- run `afmt ./file.cls` to dry-check the format result
- run `afmt -w ./file.cls` to write the format result into the file

Dry run result:
```
> afmt ./file.cls
Result 0: Ok
global class PluginDescribeResult {
    {
        [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
    }
}

Execution time: 491.772┬╡s
```


Format file result:
```
> afmt -w ./file.cls
Formatted content written back to: ./file.cls

Afmt completed successfully.

Execution time: 555.29┬╡s
```
<br>

# 📡 Technical parts

[Technical Doc](md/Technical.md)

[Config Doc](md/Settings.md)
