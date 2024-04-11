pub fn i_own_this() {
    {
        let s = String::from("hello from ownership"); // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    } // this scope is now over
      // so s is no longer valid
}

pub fn try_to_ref_old_var() {
    let s1 = String::from("hello"); // creates space for the value in the heap, and updates s1 to store the pointer
                                    // to that info...

    let s2 = s1; // creates s2 in the stack and copes the pointer to s1's heap value into s2...
                 // and s1 is no longer a valid stack item

    // println!("{}, world!", s1); <-- s1 isn't valid here anymore because it's been popped off the stack
    println!("{}, 🌎", s2);
}

pub fn clone_old_var() {
    let s1 = String::from("hola");
    let s2 = s1.clone(); // clone deep copies all the values from s1 and allocates space on the heap for it.

    // that means s1 is still valid and can be used again, as these values are like twins...
    // they look the same, but different DNA (aka, pointers in memory)
    println!("s1 = {}, s2 = {}", s1, s2);
}

pub fn copy_number() {
    let x = 5;
    let y = x; // contrary to everything else we learned in this file, numbers are okay doing this because they have a fixed
               // and known size, so copying them on the stack is "trivial".

    println!("x = {}, y = {}", x, y); // x is still valid here
}
