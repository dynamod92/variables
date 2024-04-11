pub fn passing_values_to_fn(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

pub fn const_vs_let() {
    const TEST: isize = 1;
    // const TEST = TEST + 1; // tested shadowing here and it didn't work
    // shadowing ONLY words with let, not const!
    println!("the value of TEST is: {TEST}");
}
