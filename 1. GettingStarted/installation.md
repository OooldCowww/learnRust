## Installing `rustup` on Linux or Mac
> `$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

### On macOS, you can get a C compiler by running:
> `$ xcode-select --install`

### Updating
> `$ rustup update`

### Uninstalling
> `$ rustup self uninstall`

### Local Documentation
> `$ rustup doc`

### Creating a new Cargo project
> `$ cargo new project_name`

### TOML format
[Tom's Obvious Minimal Language](https://toml.io/en/), Cargo's configuration format.
- [package]: is a section heading that indicates that the following statements are configuring a package.
- [dependencies]: is the start of a section for you to list any of your project’s dependencies.

### Building and Running a Cargo Project
> `$ cargo build`
- This command creates an executable file in target/debug/hello_cargo. Because the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
> `$ ./target/debug/hello_cargo`
- Or compile the code and then run the resultant executable all in one command:
> `$ cargo run`
- Checking your code to make sure it compiles but doesn’t produce an executable:
> `$ cargo check`

### Build for Release
> `$ cargo build --release`
