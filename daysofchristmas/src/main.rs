fn main() {
    let lyrics = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];

    for i in 0..lyrics.len() {
        println!("On the twelfth day of Christmas, my true love sent to me");

        for j in (lyrics.len() - i) - 1..lyrics.len() {
            println!("{}", lyrics[j])
        }

        println!()
    }
}
