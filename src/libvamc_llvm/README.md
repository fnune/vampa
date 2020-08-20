# libvamc_llvm

Provides tools to emit LLVM-IR from a Vampa AST, using
[Inkwell](https://github.com/TheDan64/inkwell).

## Building locally

To build this locally, a copy of LLVM 10 needs to be present in the system,
otherwise this error message will be shown by Inkwell:

```
error: No suitable version of LLVM was found system-wide or pointed
       to by LLVM_SYS_100_PREFIX.

       Consider using `llvmenv` to compile an appropriate copy of LLVM, and
       refer to the llvm-sys documentation for more information.

       llvm-sys: https://crates.io/crates/llvm-sys
       llvmenv: https://crates.io/crates/llvmenv
```

Building LLVM using `llvmenv` requires CMake:

```sh
# Ubuntu:
apt install cmake

# Arch Linux:
pacman -S cmake
```

To set up `llvmenv`:

```sh
cargo install llvmenv

# Examine the output of `llvmenv zsh` if you want:
llvmenv zsh

# Add these to .zshrc or equivalent:
export LLVMENV_RUST_BINDING=1
source <(llvmenv zsh)

# Initialize `.config/llvmenv`:
llvmenv init
```

To install LLVM 10, you can choose to use your system's installation if it uses
version 10.0.0 (for example, using `sudo apt install llvm`), or compile your
own using `llvmenv`, which may take long:

```sh
llvmenv build-entry 10.0.0
```

And then point `llvmenv` to the version you installed:

```sh
# If you're using the system installation:
llvmenv global system

# Otherwise:
llvmenv global 10.0.0
```

You may also choose to do this locally with `llvmenv local <entry>`. The
generated `.llvmenv` file is in `.gitignore`.
