use std::io::{self, Write, ErrorKind, Error};
use std::fs;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{:?}", args);
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let inventories = input.trim().split("\n\n").map(|x| {
            x.lines()
                .filter_map(|x| x.parse::<i32>().ok())
                .sum::<i32>()
        });

        println!("{:?}", inventories);

        io::stdout().write_all(format!("{}", inventories.max().unwrap()).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}
