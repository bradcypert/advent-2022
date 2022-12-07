use std::collections::HashSet;
use std::io::{self, Write, ErrorKind, Error};
use std::fs;

struct Buffer<T: std::cmp::Eq + std::hash::Hash> {
    vec: Vec<T>,
    size: usize
}

impl<T: std::cmp::Eq + std::hash::Hash> Buffer<T> {

    fn new(size: usize) -> Self {
        Buffer {
            vec: vec![],
            size
        }
    }

    fn add(&mut self, t: T) {
        if self.vec.len() == self.size {
            self.vec.remove(0);
        }
        self.vec.push(t);
    }

    fn is_full(&self) -> bool {
        return self.vec.len() == self.size
    }

    fn is_unique(&self) -> bool {
        let mut hs = HashSet::new();
        for item in self.vec.iter() {
            hs.insert(item);
        }

        return hs.len() == self.vec.len()
    }
}

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let chars = input.trim().chars();
        let mut counter = 0;
        let mut buffer = Buffer::<char>::new(14);
        for c in chars {
            buffer.add(c);
            counter += 1;
            if buffer.is_full() && buffer.is_unique() {
                break;
            }
        }

        io::stdout().write_all(format!("{}", counter).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}