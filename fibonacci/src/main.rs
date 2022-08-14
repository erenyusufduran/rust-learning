use std::io;

fn main() {
    let mut sum = 0;
    loop {
        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read line.");

        if entry == "q" {
            break;
        }

        let entry: u32 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input numbers.");
                continue;
            }
        };
        println!("Your entry: {entry}");

        for i in (1..entry).rev() {
            sum += i;
        }
        println!("Fibonacci of the index of {entry}: {sum}");
        sum = 0;
    }
}
