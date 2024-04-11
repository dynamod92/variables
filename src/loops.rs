pub fn looping_with_while() {
    let mut number = 3;

    while number != 0 {
        // while does not return a value, though it is an expression
        println!("{number}!");

        if number == 1 {
            println!("too late!")
        }

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn looping_with_loop() {
    let mut counter = 0;

    let result = 'bread: loop {
        // here, I used this *lovely* syntax to give my loop a name 🥖
        counter += 1;
        println!("👟 loop counter: {counter} ");

        if counter == 10 {
            println!("breaking 'bread 🥖");
            break 'bread counter * 2;
        }
    };

    println!("Do this in rememberance of wheat 🌾: {result}");
}

pub fn looping_over_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

pub fn looping_over_range() {
    for number in (1..4).rev() {
        // rev() makes it reverse, so it counts backwards!
        println!("{number}!");
    }
    println!("🧨");
}
