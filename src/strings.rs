pub fn string_examples() {
    let mut string: String = String::new();

    let s = "using &str";

    string.push_str("🧱");

    let string_b: String = String::from("using String::from()");

    println!("s: {}", s);

    println!("string_b: {}", string_b);
}

pub fn char_examples() {
    let c: char = 'p'; // storing a single character as a char
    println!("{}", c);

    let mut nerd: char = '🤓'; // declaring a mutable char
    println!("{}", nerd);

    nerd = '🙌';
    println!("Nerd after changing: {}", nerd);

    let z: char = 'ℤ';
    println!("Special character: {}", z);
}
