# ukeygen

Ukeygen is a cli tool who generates uniform random keys. You can choose from symbols, numerics, uppercases and lowercases characters, and also, the key size, small(8), medium(16) and large(20).

### Install

```
cargo install ukeygen
```

### Usage

```
$ ukeygen -h
An uniform random key generator

Usage: ukeygen [OPTIONS] <SIZE>

Arguments:
  <SIZE>  The key size [possible values: small, medium, large]

Options:
  -s, --symbol     Include symbols on key
  -u, --uppercase  Include uppercase alphabet characters
  -l, --lowercase  Include lowercase alphabet characters
  -n, --numeric    Include numeric characters
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version


$ ukeygen -lun medium
$ 47JSJTglQ91B6VZ3
```