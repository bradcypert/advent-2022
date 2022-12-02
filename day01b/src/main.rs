use std::io::{self, Write, ErrorKind, Error};
use std::fs;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let inventories = input.trim().split("\n\n").map(|x| {
            x.lines()
                .filter_map(|x| x.parse::<i32>().ok())
                .sum::<i32>()
        });

        let mut inven_vec = inventories.collect::<Vec<i32>>();
        inven_vec.sort();
        inven_vec.reverse();
        io::stdout().write_all(format!("{}", &inven_vec[..3].iter().sum::<i32>()).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}
