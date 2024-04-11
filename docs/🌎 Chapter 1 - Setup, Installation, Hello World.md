Every Cargo project has the following files:
- **`Cargo.toml`** - _"Tom's Obvious Minimal Language"_
	- like **`package.json`** in Node, lists **dependencies** of project, and req'd version of Rust for compilation
- **`Cargo.lock`**
	- Like **`package-lock.json`** in Node

>[!fun-fact] Cargo makes running Rust projects easy
>Cargo has cool build-in functionality to make builds, compilation verification, and running Rust programs super simple. See the list below for some common Cargo commands!

| Command                    | Usage                                                                                                                                                        |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `cargo new [project_name]` | Creates a new Cargo project at the current directory, including a `.git` config                                                                              |
| `cargo build`              | Compiles main file at current `dir` and places executable in `target/debug` path                                                                             |
| `cargo build --release`    | Updates path of executable to `target/release`                                                                                                               |
| `cargo check`              | Checks that file _could_ be compiled successfullly, but doesn't make an executable<br>                                                                       |
| `cargo fmt`                | Formats rust files                                                                                                                                           |
| `cargo run`                | Compiles _and_ builds main file at current `dir`                                                                                                             |
| `cargo update`             | Updates crates (dependencies) of the project                                                                                                                 |
| `cargo doc --open`         | Shows documentation for all installed dependencies in a super cool dynamically generated browser page 🪄                                                     |
| `rustup`                   | Checks for a newer version of Rust and installs if available                                                                                                 |
| `rustc [filename]`         | Compiles spec'd `.rs` file into an executable. Cargo is the preferred compilation technique as project become more complex. See `cargo build` or `cargo run` |


>[!question] Why use `cargo check`?
> Often, `cargo check` is much faster than `cargo build` because it skips the step of producing an executable. Use `check` periodically throughout development to make sure your project will still compile, but you won't have to wait for the build to happen!


> [!fun-fact] Fun Facts
> 1. Rust will *skip* compilation if no file changes are detected on `run` or `build`.
> 2. File names use `-` (hyphens) if containing multiple words.
> 3. Adding a `!` (bang) after a function indicates that it is a **macro**. We'll learn more about this later!
>


#intro
#commands