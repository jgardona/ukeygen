# Keygen

Keygen is a cli tool who generates uniform random keys. You can choose from symbols, numerics, uppercases and lowercases characters, and also, the key size, small(8), medium(16) and large(20).

### Install

```
cargo install keygen
```

### Usage

```
$ ./keygen -h
An uniform random key generator

Usage: keygen [OPTIONS] <SIZE>

Arguments:
  <SIZE>  The key size [possible values: small, medium, large]

Options:
  -s, --symbol     Include symbols on key
  -u, --uppercase  Include uppercase alphabet characters
  -l, --lowercase  Include lowercase alphabet characters
  -n, --numeric    Include numeric characters
  -h, --help       Print help (see more with '--help')
  -V, --version    Print version


$ ./keygen medium -lun
$ 47JSJTglQ91B6VZ3
```