# Contributing to `sw`

sw is a small and not very feature filled clone of the unix command `ls`

## Styling and formatting

Code is formatted using rustfmt and linted using clippy
You can install these with

```sh
rustup component add rustfmt
rustup component add clippy
```

Make sure you have [rust](https://www.rust-lang.org/tools/install) installed

Make sure you've given the necessary permissions to the pre-commit hooks

```sh
chmod 500 ./scripts/setup-hooks.sh ./.hooks/pre-commit.sh
```

Make sure to check for typos (preferably using [typos](https://github.com/crate-ci/typos))

Thank you for contributing
