pub fn tuples_vs_arrays() {
    let mut tup: (i32, f64, u8) = (500, 6.4, 1); // this is a tuple that stores 3 different types
    println!("Using pretty print:{:#?}", tup); // using pretty print to display the values

    tup = (600, 5.2, 2); // trying to update the values
    let (x, y, z) = tup; // using destructuring to get the values from the tup
    println!("The value of x is: {x}, y is {y}, and z is {z}");

    // yet another way of referencing values is a tuple:
    let first_value = tup.0;
    let second_value = tup.1;
    let third_value = tup.2;
    println!("{first_value}, {second_value}, {third_value}");

    // arrays
    let _a = [1, 2, 3, 4];

    // is the same as:
    let _a: [i8; 4] = [1, 2, 3, 4]; // here, we're just saying how long the array is, as well as what types of data it stores.

    // you can also initialize an array with default values:
    let array = [3; 5]; // this would make an array of 3's that has a length of 5
    println!("Printing array item at index 0: {}", array[0]);
}
