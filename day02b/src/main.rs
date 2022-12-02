use std::io::{self, Write, ErrorKind, Error};
use std::fs;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let rps_nums: Vec<Vec<i32>> = input.trim().lines().map(|x| {
            x.split(" ")
                .map(|x| match x {
                    "A" | "X" => 1,
                    "B" | "Y" => 2,
                    "C" | "Z" => 3,
                    _ => panic!(),
                }).collect()
        }).collect();

        let score = rps_nums.iter().fold(0, |acc, v| {
            let (p1, p2) = (v.first().unwrap(), v.last().unwrap());
            let win = match p1 {
                1 => 2,
                2 => 3,
                _ => 1,
            };
            let lose = match p1 {
                1 => 3,
                2 => 1,
                _ => 2
            };
            return acc + match p2 {
                1 => 0 + lose,
                2 => 3 + p1,
                _ => 6 + win,
            }
        });

        io::stdout().write_all(format!("{}", score).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}
