use index_map::IndexMap;

pub fn filter_lowercase() {
    let word = String::from("LOwER CaSE");

    for w in word
        .chars()
        .filter(|c| c.is_lowercase()) // here were filtering out any characters that aren't lowercase
        // so we get "w" and "a" as the remaining candidates
        .flat_map(|c| c.to_uppercase())
    {
        // but what filter() returns is a 3D array, like [ ["w"], ["a"] ],
        // so we use flat_map to flatten that 3D array into a 2D array of ["w","a"]
        println!("{}", w);
    }
}

pub fn iterate_over_vec() {
    let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
        .iter() // iterates over each vector entry
        .map(|x| x + 3) // adds 3 to each item
        .fold(0, |x, y| x + y); // accumulates the values together into one (like reduce in js!)
    println!("{}", x);
}

pub fn iterate_over_char_vec() {
    let x = vec!["🥹", "😇", "😂", "🥰"]
        .iter() // iterates over each vector entry
        .map(|x| format!("{}{}", x, " ➕")) // adds emoji to each item
        .collect::<String>(); // concatenates the characters together into one string

    println!("{}", x);
}

pub fn use_index_map(key: usize) {

    // this uses a function from a crate to map over an iterable and still reference the index.
    let mut emojis = IndexMap::new();

    emojis.insert("😎");
    emojis.insert("📦"); 
    emojis.insert("🦀"); 
    
    if emojis.contains_key(key) {
        let none = "none";
        let str =  emojis.get_key_value(key);

        let no_opt = match str {
            Some(s) => s,
            None => {(0, &none)},
        };

        println!("{:#?}", no_opt);
    }
}


