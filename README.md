# `elf_repack`

This utility program reads an ELF and creates a new ELF with only the necessary\* parts left over. This program is meant to be used for embedded projects where ELFs need to be as small as possible for transmission to a bootloader, for example.

\* Necessary here means:
- only `EXEC` files
- only ELF program headers are left over
- only `LOAD` segments are left over
- only `X|R`, `R|W` and `R` segments are left over

If the ELF you want to use this program with requires relocations, this will not work. If the ELF is read by a program that requires section headers, this will not work.

It's a specific program for a specific purpose :)

## Building / Installing

Assuming Cargo is set up properly, this command will install the program on the `PATH`

```
cargo install --path .
```

## CLI usage

To repack an ELF, provide the file path. If no output path is given, the ELF will be overwritten. If an output path is given then the repackaged ELF will be outputted there.

```
elf_repack some_program
```

```
elf_repack some_program -o repackaged_program
```

## License

This project is licensed under the [Apache 2.0]() license.