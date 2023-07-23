// TODO:
// - semicolon at line ending where line num > 1
// - add `And` before the last line where line num > 1

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth",
        "tenth", "eleventh", "twelve",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    // would be interesting to do this with arrays to stay on the stack
    let mut gifts_lines = vec![];

    println!("The Twelve Days of Christmas");
    println!("----------------------------");

    // for loop works here, but this looks nice :)
    (0..days.len()).for_each(|d| {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[d]
        );

        // `push` places the item at the end, `insert` at the index position
        gifts_lines.insert(0, gifts[d]);

        gifts_lines.iter().for_each(|gift| {
            println!("{}", gift);
        });

        println!();
    });
}
