use std::io::{self, Write, ErrorKind, Error};
use std::fs;

trait IncludesAnyOfVec<T> {
    fn includes_any_of_vec(&self, vec: &Vec<T>) -> bool;
}

impl IncludesAnyOfVec<u32> for Vec<u32> {
    fn includes_any_of_vec(&self, vec: &Vec<u32>) -> bool {
        vec.iter().any(|x| self.contains(x))
    }
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let lines = input.trim().lines();
        let mut counter = 0;
        for line in lines {
            let vec = line.split(",").collect::<Vec<&str>>();
            let first = vec.first().unwrap();
            let last = vec.last().unwrap();

            let fmap =  first.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            let lmap = last.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            
            let (fstart, fend) = (*fmap.first().unwrap(), *fmap.last().unwrap());
            let (lstart, lend) = (*lmap.first().unwrap(), *lmap.last().unwrap());
            let first_vec = (fstart..=fend).collect::<Vec<u32>>();
            let last_vec = (lstart..=lend).collect::<Vec<u32>>();

            if first_vec.includes_any_of_vec(&last_vec) || last_vec.includes_any_of_vec(&first_vec) {
                counter += 1;
            }
            
        }

        io::stdout().write_all(format!("{}", counter).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}