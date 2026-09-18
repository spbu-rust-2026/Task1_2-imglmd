use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut sum: u128 = 0;

    for token in input.split_whitespace() {
        if token == "-1" {
            break;
        }

        let num: u128 = match token.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("NaN");
                return;
            }
        };

        if num == 0 {
            println!("NaN");
            return;
        }

        sum += num;
    }

    println!("{}", sum);
}
