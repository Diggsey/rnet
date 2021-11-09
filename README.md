# rnet

Easily call into Rust from C# or other .net langauges.

## Usage

1. Add `rnet::root!();` to your crate.
2. Use `#[derive(Net)]` on any structs to be shared with .net.
3. Apply the `#[net]` attribute to any standalone functions
   which should be callable from .net.
4. Build your rust project as a `cdylib`.
5. Generate C# bindings for your project:
   ```
   cargo install rnet-gen
   rnet-gen "<path to .dll/.so/.dylib>" > "<path to generated file.cs>"
   ```
6. Include the C# file in your .net project.
7. Add a link to the compiled rust library to your .net project,
   and set it to "Copy if newer".
8. Optional: Configure the above steps to run automatically as
   pre-build steps.

For languages other than C#, you'll need to build the exported C# file
into its own class library, and then add a reference to that from a
project of any .net language.
