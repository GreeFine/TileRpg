# A 2D tiled game, with [Bevy](https://bevyengine.org/)

# Dev

## Dependencies

Linux: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md

Windows: Make sure to install [VS2019 build tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)

MacOS: No dependencies here

### Fast compiles
LLD linker: The Rust compiler spends a lot of time in the "link" step. LLD is much faster at linking than the default Rust linker. To install LLD, find your OS below and run the given command:

Ubuntu: `sudo apt-get install lld`</br>
Arch: `sudo pacman -S lld`</br>
Windows: Ensure you have the latest cargo-binutils</br>
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```
MacOS: Modern LLD does not yet support MacOS, but we can use zld instead: `brew install michaeleisel/zld/zld`</br>

### Nighly rust
Nightly Rust Compiler: This gives access to the latest performance improvements and "unstable" optimizations

- Install the nightly toolchain:</br>
`rustup toolchain install nightly`
- Configure your current project to use nightly (run this command within the project):</br>
`rustup override set nightly`
- OR configure cargo to use nightly for all projects -- switch back with `rustup default stable`:</br>
`rustup default nightly`

## Starting

```
cargo run
# Start in release mod. witch drasticly improve performance
cargo run --release
```


