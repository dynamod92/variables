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

pub fn enumerate_over_char_vec() {
    let x = vec!["🥹", "😇", "😂", "🥰"]
        .iter() // iterates over each vector entry
        .enumerate()
        .map(|(i, x)| format!("{}{}{}", i.to_string(), x, " ➕")) // adds emoji to each item
        .collect::<String>(); // concatenates the characters together into one string

    println!("{}", x);
}

// this is hackier than enumerate() but still works
pub fn use_index_map(key: usize) {
    // this uses a function from a crate to map over an iterable and still reference the index.
    let mut emojis = IndexMap::new();

    let word = &"test".to_string();

    emojis.insert("😎".to_string());
    emojis.insert("📦".to_string());
    emojis.insert("🦀".to_string());

    // quick way to get a value at index, or return something else if it doesn't exist
    let x = match emojis.get_key_value(key) {
        Some(s) => Some((s.0, s.1.to_owned())),
        None => Some((0, word.to_owned())),
    };

    println!("Doing a match: {:?}", x);

    // this check doesn't ensure anything like it would in TS 😭, so it's better to omit it entirely.
    // I'm leaving it here though with this note to remind myself as I will definitely forget
    // in the future.
    if emojis.contains_key(key) {
        let none = "missing emoji".to_string();
        let str = emojis.get_key_value(key); // this is repetitive in that we still can't be sure this index
                                             // exists

        let no_opt = match str {
            Some(s) => (s.0, s.1.to_owned()), // if it *does* exist
            None => (0, none),                // if not 💀
        };

        println!("{:#?}", no_opt);
    }

    // This looks wild, right?
    // This function accepts a pointer to a borrowed map (emojis) as well as a new emoji to add to it.
    // Once they're added to the map, it returns the value it just added
    fn update_emojis(map: &mut IndexMap<String>, word: &str) -> Option<(usize, String)> {
        // add the new item to the map
        let last_index = map.insert(word.to_string());
        println!("adding new value: {:?}", new_item);

        // fetch the newly created item, and unwrap it to get the values
        let new_item = map.get_key_value(last_index).unwrap();
        
        // return the newly created item, but with the unborrowed map value
        Some((new_item.0, new_item.1.to_owned()))
    }

    let str = emojis
        .get_key_value(key)
        .and_then(|x| Some((x.0, x.1.to_owned())))
        // .or(Some((0, "or".to_string()))) // this was hard, I'm so proud haha
        .or_else(|| {
            // update the map to include the text
            update_emojis(&mut emojis, "🔥")
        }) // also hard, but easier after understanding or()!
        .expect("how could you not know!?"); // this lets us say "i expect one of the "or" or "or_else" backups
                                             // to handle it if it doesn't exist, but if not, handle it like this."

    println!("fetching emoji from map: {:?}", str);
}
