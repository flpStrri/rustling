use std::io;

fn main() {
    println!("What is the temperature in Fahrenheit?");

    let mut temperature= String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Failed to read number");

    let celcius = (temperature - 32.0) * (5.0/9.0);
    println!("So... its {celcius}C in the right metric system");
}