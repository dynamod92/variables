## Using Imports
To import functionality into your file, add `use` at the top of the file (like `import` in `.js`).

```rust
// top of main.rs
use std::io;
```

### Prelude vs non-Prelude
The *Prelude* is a set of set of items automatically made available within the context of an application, such as `fn` or `String` (see [docs](https://doc.rust-lang.org/std/prelude/index.html) for more info).

If something you want to use isn't included in _Prelude_, use the keyword `use` to import it. 

>[!fun-fact]
>`use` is from the Prelude 🪄

## Syntax

| Keyword           | Purpose                                                               | Example                                                                 |
| ----------------- | --------------------------------------------------------------------- | ----------------------------------------------------------------------- |
| `let`             | Defines new variable, immutable by default                            | `let apples = 5;`                                                       |
| `mut`             | Makes variable mutable                                                | `let mut bananas = 5;`                                                  |
| `//`              | Syntax for line comment                                               | `// this is a comment`                                                  |
| `::`              | Indicates that a function belongs to a class                          | `String::new // String is class, new is function`                       |
| `String::new()`   | Creates an new instance of `String`                                   | `let mut guess = String::new();`                                        |
| `io::stdin()`     | Creates a new instance of `Stdin`                                     | `io::stdin().read_line(&mut guess)`<br>                                 |
| `&`               | Indicates a pointer reference                                         | `&mut guess`                                                            |
| `&mut [var_name]` | Passes a pointer to a variable in which a calc'd val should be stored | `// in the line above, the result of read_line will be stored in guess` |
| `.expect`         | Handles a failure after a function call returns an `Err` result type  | `.expect("Failed to read line");`                                       |
| `{}`              | Allow value injection in string interpolation                         | `println!("You guessed: {guess}");`                                     |
| `start..=end`     | Creates a range of values, inclusive of start and end bounds          | `1..=100`                                                               |
> [!tip] **Using `new`**
> `new` is a common function on many classes as it's the commonplace pattern for instantiation. To call the `new` function on a class, reference the class, then append `::new` like so:
> ```rust
> let mut guess = String::new()
> ```


## Program Overview
```rust
fn main() {
    println!("Guess the number!"); // prints message and prompts user for input
    println!("Please input your guess.");

    let mut guess = String::new(); // defines "guess" as a mutable variable, then calls String::new() to create a new instance of a String.

    io::stdin()
        .read_line(&mut guess) // uses the read_line() func on an instance of Stdin, then stores the Result of read_line at the pointer for guess.
        
        .expect("Failed to read line"); // if the return type of Result is Err, then expect catches that and throws the message provided as an error.

    println!("You guessed: {guess}"); // prints the value the user entered
}
```


>[!question] **How does `Result` know what to return?**
>The `Result` type is an **enum** that can be one of 2 values: `Err` or `Ok`.
>Here's the actual type definition for a better visual:
>```rust
>enum Result<T, E> {
>    Ok(T), // T is the type of value we expect to return if the function doesn't error (i.e. "String")
>    Err(E),
>}
>```
> If the value of `Result` is `Err`, then it must be caught with `.expect()` so that a message can be displayed indicating the error. If you don’t call `expect`, the program will compile, but you’ll get a warning than an error could occur at runtime.
#### Dependencies and Crates
- **Binary** Crate - Compiled application that can be run
- **Library** Crate - Extensibility that can be imported into Binary Crates to add functionality

**Adding Dependencies**
To add a new dependency, update the `Cargo.toml` file for your project to include the crate name:
```toml
[dependencies]
rand = "0.8.5"
```

Next, run `cargo build` or `cargo run` to download the new dependency, as well as any dependencies _that_ dependency has.


>[!tip] Need external functionality?
> In Rust, packages of code are referred to as _crates_ and can be imported using Cargo's package management tooling. As long as they're listed under `[dependencies]` in `Cargo.toml` they'll be fetch and imported when `cargo run` or `cargo build` is run.


**The `rand` Dependency**
- `rand::Rng` - `Rng` trait defines methods that can be used for `random number generation`.
	- For us to use any methods on the `Rng` trait, that has to be in scope. We add it to the scope with `use`, like so: `use rand::Rng;`

**Comparing Results Using `match`**
```rust
 match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

`Ordering` is an `Enum` that returns either `Less`, `Greater`, or `Equal`. This `match` statement compares the result of `guess.cmp(val)` and prints a message depending on the type returned. This comparison is possible because `Ordering` can only ever return one of these 3 values. 

>[!fun-fact]
>Pattern matching techniques like `match` allow for deciding behavior based on *return types* instead of *return values*.




