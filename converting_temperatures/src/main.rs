use std::io;

fn main() {
    println!("Enter a temperature: ");

    loop {
        let mut temperature_in_celcius = String::new();

        io::stdin()
            .read_line(&mut temperature_in_celcius)
            .expect("Failed to read line.");

        let temperature_in_celcius: u32 = match temperature_in_celcius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input numbers.");
                continue;
            }
        };

        println!("{}", temperature_in_celcius);

        let fahrenheit_temperature: u32 = (temperature_in_celcius * 9 / 5) + 32;
        println!("{}", fahrenheit_temperature);
        break;
    }
}
