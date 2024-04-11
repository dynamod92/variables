pub fn number_examples() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let dec: u32 = 98_222;
    println!("dec: {dec}");

    let dec: u32 = 0b1111_0000;
    println!("dec: {dec}");

    let dec: u32 = 0xff;
    println!("dec: {dec}");
}

pub fn math_examples() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient: f32 = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("the value of all thes variables is: [ quotient: {quotient}, sum: {sum}, difference: {difference}, product: {product}, truncated: {truncated}, remainder: {remainder}
]");
}
