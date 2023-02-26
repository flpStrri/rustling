use std::io;

fn fahrenheit_to_celcius(fahrenheit_temperature: f64) -> f64 {
    (fahrenheit_temperature - 32.0) * (5.0/9.0)
}

fn main() {
    println!("What is the temperature in Fahrenheit?");

    let mut temperature= String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Failed to read number");

    let celcius = fahrenheit_to_celcius(temperature);
    println!("So... its {celcius}C in the right metric system");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(fahrenheit_to_celcius(32.0), 0.0);
    }
}