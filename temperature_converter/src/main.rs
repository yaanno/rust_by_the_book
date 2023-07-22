fn main() {
    let c = 4.5;
    let f = to_fahrenheit(c);
    println!("{}C is {:.2}F", c, f);

    let f = 5.4;
    let c = to_celsius(f);
    println!("{}F is {:.2}C", f, c);
}

fn to_fahrenheit(value: f32) -> f32 {
    value / 1.8 + 32.0
}

fn to_celsius(value: f32) -> f32 {
    value - 32.0 / 1.8
}
