## 🧹 Memory Cleanup
Rust cleans up kind of like how Robin and I clean up - as soon as something isn't being used anymore, put it away (aka, toss the value and free up the memory). Other languages either have **garbage collection (gc)** to handle this or expect the engineer to manage/free up memory as needed, but Rust favors knowing how to handle memory *at compile time*. This is done with a set of rules and compiler checks, and prevents runtime slowdowns.

##  Stack vs Heap

### 🥞 Stack
The **call stack** is where tasks are stored *last in, first out.* Items placed here have a fixed size that is known at compile time. If the size can't be known at compile time, you gotta move it to the heap. You can **push**➕ and **pop**➖items in the stack.

>[!tip] 🍽️ Plates in the cabinet
>The Rust book gives stacking plates as an example of how the stack works: When you put plates away, you put them on top, and when you take a plate, you take it from the top.
>
>I want to expand on this by talking about the height of the cabinet where the plates are stored. If you try to stack too many plates than can fit in the cabinet, you get **stack overflow**. 
>
>Additionally, if you had a plate that was under a magical grow and shrink spell that you couldn't predict, that plate could push all the other plates out of the cabinet and cause a stack overflow. That's why that kind of plate would go in the heap!

### 💾 Heap
The heap is what it sounds like - a pile of varying things. The size of each item doesn't need to be known at compile time, as anything that goes into the heap will just be put wherever the **allocator** finds space for it. Once something is stored there, you get a **pointer** to it for referencing later. 

>[!tip] Read speed
>Accessing data on the stack is *faster* than reading from the heap because the heap has to follow a pointer to get to the data instead of just taking the top plate. 

## Ownership
### Rules:
1. Each value has an ***owner***
2. There can only ever be ***one*** owner at a time
3. When the owner goes out of scope of the value, the value will be ***dropped***

```rust
fn main() {
	// s is not valid here, it’s not yet declared
	let s = "hello";   // s is valid from this point forward
	// this lives on the stack since it's a string literal, aka
	// the size is known at compile time and doens't change

    let str = String::from("howdy"); // this one will get stored on the heap

	// do stuff with s
}                      // this scope is now over, and s is no longer valid
```

>[!info] Some things to note from the above example
>- when `s` comes ***into scope***, it's **valid** (aka you can reference it, etc)
>- when `s` goes ***out of scope*** (when the curly braces close and there are no more references to `s`) it becomes **invalid**


## Copying Data

