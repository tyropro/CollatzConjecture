use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Collatz Conjecture \nEnter a number:");

    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).expect("");
    input = input.trim().to_owned();
    if input.len() == 1 && input == "q" {
        return;
    }

    let mut data_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("data.csv")
        .expect("cannot open file");

    let mut num: u128 = input.parse::<u128>().unwrap();
    let mut iter: u64 = 1;

    let data = format!("ID, Number\n{}, {}\n", iter, num);

    data_file
        .write(data.as_bytes())
        .expect("cannot write to file");

    loop {
        if num % 2 == 1 {
            num *= 3;
            num += 1;
        } else {
            num /= 2;
        }

        let data = format!("{}, {}\n", iter, num);
        data_file
            .write(data.as_bytes())
            .expect("cannot write to file");

        if num == 1 {
            break;
        }

        iter += 1;
    }
}
