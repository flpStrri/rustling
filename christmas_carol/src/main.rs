fn main() {
    println!("THE TWELVE DAY'S OF CHRISTMAS");
    println!("");
    for day_num in 0..12 {
        sing_day(day_num + 1);
    }
}


fn sing_day(num: usize) {
    let ordinal_day = ordinal(num);
    println!("On the {ordinal_day} day of Christmas my true love sent to me");
    sign_nth_day(num);
    println!("");
}

fn sing_day_one_for(num: usize) {
    match num {
        1 => println!("A partridge in a pear tree."),
        _ => println!("And a partridge in a pear tree."),
    }
}

fn sign_nth_day(num: usize) {
    let verses_two_to_twelve = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five gold rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
    ];

    for day in &verses_two_to_twelve[12-num..] {
        println!("{}", day);
    }
    sing_day_one_for(num);
}


pub fn ordinal(num: usize) -> String {
    let ones = num % 10;
    let tens = (num / 10) % 10;

    match (ones, tens) {
        (_, 1) => format!("{}th", num),
        (1, _) => format!("{}st", num),
        (2, _) => format!("{}nd", num),
        (3, _) => format!("{}rd", num),
        (_, _) => format!("{}th", num),
    }
}
