use std::io;

fn main() {
    let mut sum: i128 = 0;
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        let num: i128 = match line.trim().parse() {
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
