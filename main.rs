use color_eyre::eyre::{self, Result};
use std::io;

#[derive(Debug, Copy, Clone)]
struct Square {
    num: u8,
    grain: u128,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let mut chessboard: Vec<Square> = Vec::new();
    let mut grain: u128 = 1;
    let mut total: Vec<u128> = Vec::new();
    for num in 1..65 {
        total.push(grain.clone());
        let square = Square { num, grain };
        grain *= 2;
        chessboard.push(square);
    }

    let total: u128 = total.iter().sum();
    loop {
        println!("Please choose one of the options:\r\n1) Show the number of grains on a given square\r\n2) Show the total number of grains");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();
        let keywords = ["square", "given", "the number"];
        if keywords.iter().any(|&keyword| input.contains(keyword)) {
            println!("Enter the number of a given square");
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let num: u8 = input.trim().parse().expect("Failed to parse the input");
            if num > 64 || num < 1 {
                panic!("Square must be between 1 and 64");
            }
            if let Some(sq) = chessboard.iter().find(|&sq| sq.num == num) {
                    println!("There are {} grains on the square {}", sq.grain, sq.num);
            }
            break;
        } else if input.contains("total") {
            println!("The total number of grains: {}", &total);
            break;
        } else {
            println!("Invalid input, try again");
            continue;
        }
    }
    Ok(())
}
