use std::io;

fn main() {
    let mut sum: i128 = 0;
    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break;
        }

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let num: i128 = match line.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("NaN");
                return;
            }
        };
        if num == -1 {
            break;
        }
        if num <= 0 {
            println!("NaN");
            return;
        }
        sum += num
    }
    println!("{}", sum)
}