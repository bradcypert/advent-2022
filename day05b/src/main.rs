use std::cell::RefCell;
use std::io::{self, Write, ErrorKind, Error};
use std::fs;

use std::rc::Rc;

fn main() -> io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    if let Some(file) = args.get(1) {
        let input = fs::read_to_string(&file).unwrap();
        let split_input = input.split("\n\n").collect::<Vec<&str>>();
        let grid_str = *split_input.first().unwrap();
        let columns = grid_str.lines().last().unwrap().split("").filter_map(|x| x.parse::<u32>().ok());
        let col_count = columns.last().unwrap();

        let mut grid: Vec<Rc<RefCell<Vec<String>>>> = vec![];
        for _ in 0..col_count {
            grid.push(Rc::new(RefCell::new(vec![])));
        }
        let grid_lines = grid_str.lines().collect::<Vec<&str>>();


        // this gives us a vec of vecs representing the grid
        for index in 0..col_count {
            for line in grid_lines.iter() {
                let line_data = line.chars().collect::<Vec<char>>();
                let item = line_data.get(usize::try_from(4 * index + 1).unwrap()).unwrap();
                if item.is_alphabetic() {
                    let sub_vec = Rc::clone(grid.get_mut(index as usize).unwrap());
                    (*sub_vec).borrow_mut().push(item.to_string().clone());
                }
            }

            grid.get_mut(index as usize).unwrap().borrow_mut().reverse();
        }

        // now iterate through instructions and do the work
        // move X from Y to Z 
        for line in split_input.last().unwrap().trim().clone().lines() {
            let instructions = line.split(" ").filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>();
            let (count, from, to) = (instructions[0], instructions[1]-1, instructions[2]-1);
            let from_vec = Rc::clone(grid.get(from as usize).unwrap());
            let to_vec = Rc::clone(grid.get(to as usize).unwrap());
            let mut items: Vec<String> = vec![];
            for _ in 0..count {
                let item = from_vec.borrow_mut().pop().unwrap();
                items.push(item)
            }
            items.reverse();
            to_vec.borrow_mut().append(&mut items);
        }

        let last = grid.iter().map(|row| row.clone().borrow().last().unwrap().clone()).collect::<Vec<String>>().join("");

        io::stdout().write_all(format!("{}", last).as_bytes())?;
        Ok(())
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "You must provide a filename for the input."))
    }
}