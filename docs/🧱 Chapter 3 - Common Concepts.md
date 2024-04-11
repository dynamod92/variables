## Variables and Mutability
Rust prefers that variables be immutable, but does still allow an easy way to make values mutable. It's a subtle nudge towards a coding best practice of not re-using variables, and preventing hard-to-diagnose side effects.

### `let`
The keyword `let` defines a variable that is immutable by default, but can be made mutable with the keyword `mut`.

```rust
let bananas = 5; // declares an immutable
let mut apples = 5; // declares a mutable variable

apples = 6; //overrides previous value
```

### Shadowing 👻
You can declare a new variable with the same name as an existing one, and that's called **Shadowing**. All that means is that the original value of the variable will be overshadowed with the second. 

>[!question] Why the heck would you do that? 
>I'm still figuring it out, but it looks like it's a way to keep the original variable as immutable, while creating new instances of the variable with different values.

Shadowing is a workaround to make variables mutable that weren't previously. Check out the example below for how that works:

```rust 
let bananas = 5; // declares an immutable
let bananas = bananas + 2; // declares a new variable that shadows the old one
println!(bananas) // this equals 7 🍌
```

While both variables have the name `bananas`, they occupy different places in memory, and therefor can store different values.

>[!info] Shadowing only works for `let`
>Variables declared as constsants with `const` cannot be shadowed, and you will get an error like this:
>
>```shell
> error[E042]: the name `TEST` is defined multiple times
  >-->src/main.rs:10:5
>    |
>9  |     const TEST = 1;
   |     --------------- previous definition of the value `TEST` here
>10 |     const TEST = TEST + 1;
>    |     ^^^^^^^^^^^^^^^^^^^^^^ `TEST` redefined here
>    |
> = note: `TEST` must be defined only once in the value namespace of this block

#### `const`
Declares a permanently immutable variable. Syntax for variables using `const` includes `ALL_UPPERCASE_DIVIDED_BY_UNDERSCORES`.

```rust
const THIS_IS_IMMUTABLE: u32 = 1024;
// note that const requires an annotation of its type (u32) at declaration
```

##  Data Types

| Type                                      | Notes                                                                         |
| ----------------------------------------- | ----------------------------------------------------------------------------- |
| `u8`, `u16`,`u32`, `u64`, `u128`, `usize` | Unsigned integer, can hold types like hex values, decimals, and a few others. |
| `i8`, `i16`,`i32`, `i64`, `i128`, `isize` | Signed integer, can hold types like hex values, decimals, and a few others.   |
| `f32`                                     | floating point to 1 decimal                                                   |
| `f64`                                     | floating point to 2 decimals                                                  |

>[!tip] Operators and Symbols
>For a list of all operators for mathematical operations in Rust, check out the docs [here!](https://doc.rust-lang.org/book/appendix-02-operators.html)

#### Signed vs Unsigned Integers
_Signed_ and _unsigned_ refer to whether it’s possible for the number to be negative—in other words, **whether the number needs to have a sign with it (signed)** or whether it will **only ever be positive and can therefore be represented without a sign** (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign.

The "8" in `i8` indicates the maximum size of data that can be stored in the variable. That numeric indicator says that the size is  **-(2<sup>n - 1</sup>) to 2<sup>n - 1 </sup>- 1 inclusive** of the bounds. So in the case of `i8`, `n` is 8, and `n - 1` is 7. So the total size the integer can hold is **2<sup>7</sup> - 1**:
		`2 x 2 x 2 x 2 x 2 x 2 x 2 - 1 = 127` top of range
	or **-(2<sup>7</sup>)**:
		`-(2 x 2 x 2 x 2 x 2 x 2 x 2) -1 = -128` bottom of range

In the case of an _unsigned_ number the range starts from zero, and ends at a bounds-inclusive range of **(0 to 2<sup>n - 1</sup>).** Using `u8` as an example, the allowed size would be would be **2<sup>8</sup>-1**, which is 255.

>[!question] Why is *unsigned* 2<sup>n</sup> when *signed* is 2<sup>n-1</sup>?
> It takes an **entire bit** to mark whether a number is positive or negative (enough space to store a dash!). Unsigned numbers get to hold more data in that bit, while unsigned numbers have to reserve that space for an indicator of whether or not the number is negative.

When in `debug` mode, Rust will panic if you try to store a value that's too big for the integer type you've specified. If you build a release with the `--release` flag, your application will no longer panic at runtime if trying to store a number that's too big for a spec'd type. Instead, it will "wrap" the value around. 

For example, in the case of a `u8` (whose max is 255), the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. 

To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

- Wrap in all modes with the `wrapping_*` methods, such as `wrapping_add`.
- Return the `None` value if there is overflow with the `checked_*` methods.
- Return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
- Saturate at the value’s minimum or maximum values with the `saturating_*`methods.

>[!info] Navigating Rust docs
>I had a hard time finding `wrapping_add`, but it's because these wrapping methods mentioned above live [_with the datatypes_](https://doc.rust-lang.org/std/primitive.i64.html#method.wrapping_add) in the docs, instead of in a central place. Silly but oh well lol


>[!question] What if I need integer space to match my architecture?
>The **`isize`** and **`usize`** types depend on the architecture of the computer your program is running on, which is denoted in the table as **`arch`**: 
>	- 64 bits if you’re on a 64-bit architecture
>	- 32 bits if you’re on a 32-bit architecture


>[!fun-fact] Using `_` to read large numbers quickly 
>Number literals can also use `_` as a visual separator to make the number easier to read, such as `1_000`, which will have the same value as if you had specified `1000`

### `String`
Use double quotes to specify `String` values:
```rust
let s: String = "this is a string!";
```
##  `Char` 
Specify `Char` literals with single quotes, as opposed to string literals, which use double quotes:
```rust
let c: Char = 'z'; // includes explicit type annotation 
```
The `char` type is **4 bytes**, so it can represent a lot more than just ASCII (that means you can use emojis! 🤩)

### 🗂️ Tuples
Tuples can hold a fixed and specified number of values of any type.

```rust
    // here, the values have their type indicated at instantiation
    let tup: (i32, f64, u8) = (500, 6.4, 1); 

    // in this case, the value types will be inferred at compile time.
	let vague_tup = (500, 6.4, 1);

    // tuples values can be updated by adding `mut`
    let mut tup_mutable = (1, 2, 3);

    // here we updated the values, but we can't update the size
    tup_mutable = (4, 5, 6); 

    // you can reference tuple values in a few ways: 
    // 1. destructuring the tuple
    let (x, y, z) = tup;
    
    // 2. direct reference by item index
    println!(tup.0)
```

>[!tip] Tuple declaration and mutability
>Tuples have a fixed length. Once declared, they cannot grow or shrink in size. At the time of declaring a tuple, you should specify **either** or **both** of the following:
>	- _values_ they'll store, or
>	- _types_ of the values they'll store

### 📚 Arrays
Arrays are useful when you want your data allocated on the stack rather than the heap.
```rust
    let a = [1, 2, 3, 4, 5]; // this declares an array and sets its init vals
```


### **Expressions vs Statements**

| Expressions | return a value       |
| ----------- | -------------------- |
| Statements  | don't return a value |


>[!love] Quote from the [docs](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability): 
>*"Compiler errors can be frustrating, but really they only mean your program isn’t safely doing what you want it to do yet; they **do not mean that you’re not a good programmer**! Experienced Rustaceans still get compiler errors."* 
>
>Be kind to yourself through this journey of learning. 🦋

#shadowing #immutable #mutable #types #math #functions


## Functions
```rust
fn five() -> i32 { // the arrow indicates that the function is an expression
    5   // this is the value that will be returned when the function is called
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
```


## Loops
`for`, `while`, `loop`

