# libvamc_llvm

Provides tools to emit LLVM-IR from a Vampa AST, using
[Inkwell](https://github.com/TheDan64/inkwell).

## Building

LLVM is supplied by the Nix flake at the repository root, so there is nothing to
install by hand. See the [top-level README](/README.md) for the development
setup, then build from a dev shell:

```sh
nix develop -c cargo build
```

The required `LLVM_SYS_*_PREFIX` environment variable is set automatically inside
the dev shell.

## Updating snapshots

The codegen tests are [insta](https://insta.rs) snapshots. After an intentional
change to the emitted IR, review and accept the new snapshots with:

```sh
nix develop -c cargo insta review
```
