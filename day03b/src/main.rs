use std::io::{self, Write, ErrorKind, Error};
use std::fs;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let lines = input.trim().lines();
        let mut priority = 0;
        let lines_as_strings = lines.collect::<Vec<&str>>();
        let chunks = lines_as_strings.chunks(3);
        for chunk in chunks {
            for c in chunk[0].trim().chars() {
                if chunk[1].contains(c) && chunk[2].contains(c) {
                    priority += get_priority(c);
                    break;
                }
            }
        }

        io::stdout().write_all(format!("{}", priority).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}


fn get_priority(c: char) -> u32 {
    let value = c as u32;
    if value <= 90 {
        // uppercase
        value - 38
    } else {
        // lowercase
         value - 96
    }
}