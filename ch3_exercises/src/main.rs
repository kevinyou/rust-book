fn main() {
    println!("Hello, world!");

    let current_temp = 55.0;

    println!("Current temp in F: {current_temp}");
    let current_temp = farenheit_to_celsius(current_temp);
    println!("Current temp in C: {current_temp}");
    let current_temp = celsius_to_farenheit(current_temp);
    println!("Current temp in F: {current_temp}");

    for i in 0..10 {
        let fib_num = generate_nth_fibonacci(i);
        println!("{fib_num}");
    }

    print_days_of_christmas();
}

fn farenheit_to_celsius(deg: f64) -> f64 {
    (deg - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(deg: f64) -> f64 {
    (deg * 9.0 / 5.0) + 32.0
}

fn generate_nth_fibonacci(index: u8) -> u32 {
    if index <= 1 {
        return 1;
    }
    generate_nth_fibonacci(index - 1) + 
    generate_nth_fibonacci(index - 2)
}

fn print_days_of_christmas() {
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];
    for day_number in 1..12 {
        println!("On day {day_number} of Christmas my true love gave to me");

        let mut i = day_number;
        while i > 0 {
            let verse = verses[i];
            println!("{verse}");
            i -= 1;
        }

        println!("A partridge in a pear tree");
    }
}
