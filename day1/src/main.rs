use std::fs::File;
use std::io::{self, prelude::*};
use std::cell::Cell;
use std::collections::HashSet;

struct Elf {
    items: Vec<u32>,
    _total_calories: Cell<Option<u32>>,
}

impl Elf {
    fn new() -> Elf {
        Elf {
            items: Vec::new(),
            _total_calories: Cell::new(None),
        }

    }

    fn total_calories(self: &Self) -> u32 {
        match self._total_calories.take() {
            None => {
                self._total_calories.replace(Some(self.items.iter().sum()));
                self._total_calories.get().unwrap()
            },
            Some(n) => n
        }
    }
}

fn main() {
    let mut args: std::env::Args = std::env::args();
    args.next();


    let file_path: String = match args.next() {
        Option::Some(s) => s,
        Option::None => {
            println!("You must provide an input file.");
            std::process::exit(1);
        }
    };

    let file = match File::open(&file_path) {
        Result::Ok(f) => f,
        Result::Err(_) => {
            println!("unable to read file at {file_path}");
            std::process::exit(1);
        }
    };
    let mut reader = io::BufReader::new(file);
    let mut line_buffer = String::new();

    let mut elves: Vec<Elf> = Vec::new();

    elves.push(Elf::new());
    let mut current_elf: &mut Elf = elves.last_mut().unwrap();

    while let Ok(_) = {
        line_buffer.clear();
        reader.read_line(&mut line_buffer)
    } {
        let is_empty: bool = match line_buffer.chars().nth(0) {
            Some('\n') => true,
            Some(_) => false,
            None => break,
        };

        if is_empty {
            elves.push(Elf::new());
            current_elf = elves.last_mut().unwrap();
            continue;
        }

        let calories: u32 = match line_buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse '{line_buffer}' as an integer.");
                std::process::exit(1);
            } 
        };

        current_elf.items.push(calories)
    };

    println!("{}", top_n(3, &elves))
}

fn top_n(n: usize, elves: &Vec<Elf>) -> u32 {
    let mut best_elf_indexes: HashSet<usize> = HashSet::new();

    let mut top_n_cals: u32 = 0;
    
    for _ in 0..n {
        let mut best_elf_index: usize = 0;
        for (i, elf) in elves.iter().enumerate() {
            if best_elf_indexes.contains(&i) {
                continue;
            }

            if elf.total_calories() > elves[best_elf_index].total_calories() {
                best_elf_index = i
            }
        }
        best_elf_indexes.insert(best_elf_index);
        top_n_cals += elves[best_elf_index].total_calories()
    };

    return top_n_cals
}
