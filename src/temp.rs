pub fn c_to_f(number: f32) -> f32 {
    let calc = number * 1.8 + 32.0;
    println!("{number} f = {calc} c");
    calc
}

pub fn f_to_c(number: f32) -> f32 {
    let baseline = number - 32.0;

    let calc = baseline * (5.0 / 9.0);

    println!("{number} f = {calc} c");
    calc
}
