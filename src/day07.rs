use std::{vec, cell::RefCell, rc::Rc, borrow::Borrow, process::Output};

use aoc::read_inputs;

type DirectoryRef = Rc<RefCell<Directory>>;

struct File {
    name: String,
    size: u32
}

struct Directory {
    name: String,
    parent: Option<DirectoryRef>,
    subdirs: Vec<DirectoryRef>,
    files: Vec<File>,
    size: u32
}

impl Directory {
    pub fn new(name: String, parent: Option<DirectoryRef>) -> Self {
        return Self { 
            name: name,
            parent: parent,
            subdirs: vec![],
            files: vec![],
            size: 0
        };
    }

    pub fn new_root() -> Self {
        return Self {
            name: String::from("/"),
            parent: None,
            subdirs: vec![],
            files: vec![],
            size: 0
        };
    }

    pub fn is_subdir(&self, name: &str) -> bool {
        for dir in self.subdirs.iter() {
            if &dir.try_borrow().unwrap().name == name {
                return true;
            }
        }

        return false;
    }

    pub fn get_subdir(&mut self, name: &str) -> Option<DirectoryRef> {

        for dir in self.subdirs.iter() {
            if &dir.try_borrow().unwrap().name == name {
                return Some(dir.clone());
            }
        }
        return None;
    }

    pub fn calculate_sizes(&mut self) {
        self.size = 0;

        for dir in self.subdirs.iter() {
            dir.borrow_mut().calculate_sizes();
            self.size += dir.try_borrow().unwrap().size;
        }

        for f in self.files.iter() {
            self.size += f.size;
        }

        // println!("Dir name: '{0}'\tSize: {1}", self.name, self.size);
    }

    pub fn print_tree(&self, level: Option<u32>) {
        let lvlprint = |lvl| {
            for _ in 0..lvl {
                print!(" ");
            }
        };

        let lvl = level.unwrap_or(0);

        lvlprint(lvl);

        if lvl == 0 {
            println!("- {0} (dir)", self.name);
        }

        for dir in self.subdirs.iter() {
            lvlprint(lvl+1);
            println!("- {0} (dir)", &dir.try_borrow().unwrap().name);
            dir.try_borrow().unwrap().print_tree(Some(lvl+1));
        }
        
        
        for f in self.files.iter() {
            lvlprint(lvl+1);
            println!("- {0} (file, size={1})", f.name, f.size);
        }

    }

    pub fn get_dirs_le_than(self_: DirectoryRef, size: u32, out: &mut Vec<DirectoryRef>) {
        if self_.try_borrow().unwrap().size <= size {
            out.push(self_.clone());
        }
        
        for dir in self_.try_borrow().unwrap().subdirs.iter() {
            Self::get_dirs_le_than(dir.clone(), size, out);
        }
    }

    pub fn get_dirs_ge_than(self_: DirectoryRef, size: u32, out: &mut Vec<DirectoryRef>) {
        if self_.try_borrow().unwrap().size >= size {
            out.push(self_.clone());
        }

        for dir in self_.try_borrow().unwrap().subdirs.iter() {
            Self::get_dirs_ge_than(dir.clone(), size, out);
        }
    }
}

fn create_filesystem_tree(terminal_output: &Vec<String>) -> DirectoryRef {
    let root = Rc::new(RefCell::new(Directory::new_root()));

    let mut current_head: DirectoryRef = root.clone();

    for line in terminal_output.iter() {
        // println!("{}", line);
        let mut line_iter = line.split(' ');

        let first_token = line_iter.next().unwrap();
        if first_token == "$" {
            let command = line_iter.next().unwrap();
            
            if command == "cd" {
                let dirname = line_iter.next().unwrap();
                if dirname == "/" { 
                    current_head = root.clone();
                    continue;
                }

                if dirname == ".." {
                    let parent_clone = current_head.try_borrow().unwrap().parent.as_ref().unwrap().clone();
                    current_head = parent_clone;
                    continue;
                }

                let child = current_head.try_borrow_mut().unwrap()
                    .get_subdir(dirname).unwrap().clone();

                current_head = child.clone();
            }
            continue;
        }

        if first_token == "dir" {
            let dirname = line_iter.next().unwrap();
            
            if !current_head.try_borrow().unwrap().is_subdir(dirname) {
                let new_dir = 
                    Rc::new(
                        RefCell::new(
                            Directory::new(
                                String::from(dirname),
                                Some(current_head.clone())
                            )
                        )
                    );

                current_head.try_borrow_mut().unwrap().subdirs.push(new_dir);
            }
            continue;
        }
        
        let size = u32::from_str_radix(first_token, 10).unwrap();
        let filename = line_iter.next().unwrap();
        current_head.try_borrow_mut().unwrap().files.push(
            File {
                name: String::from(filename),
                size: size
            }
        );
    }
    return root;
}


fn part1(input: &Vec<String>) {
    let root = create_filesystem_tree(&input);
    root.borrow_mut().calculate_sizes();
    // root.try_borrow().unwrap().print_tree(None);
    
    let mut out: Vec<DirectoryRef> = vec![];

    Directory::get_dirs_le_than(root.clone(), 100000, &mut out);

    let mut acc = 0u32;
    for dir in out.iter() {
        acc += dir.try_borrow().unwrap().size;
    }
    println!("{}", acc);
}

fn part2(input: &Vec<String>) {
    let root = create_filesystem_tree(&input);
    root.borrow_mut().calculate_sizes();
    // root.try_borrow().unwrap().print_tree(None);
    
    const TOTAL_SPACE: u32 = 70_000_000;
    const MIN_UNUSED_SPACE: u32 = 30_000_000;

    let used_space = root.try_borrow().unwrap().size;
    let min_space_to_free = MIN_UNUSED_SPACE - (TOTAL_SPACE - used_space);

    let mut out: Vec<DirectoryRef> = vec![];

    Directory::get_dirs_ge_than(root.clone(), min_space_to_free, &mut out);

    let mut smallest_possible_dir: DirectoryRef = root.clone();
    for dir in out.iter() {
        // println!("{:?}", dir.try_borrow().unwrap().name);
        if dir.try_borrow().unwrap().size < smallest_possible_dir.try_borrow().unwrap().size {
            smallest_possible_dir = dir.clone();
        }
    }
    println!("{}", smallest_possible_dir.try_borrow().unwrap().size);
}

fn main() {
    let day = 7;
    #[cfg(debug_assertions)]
    let file_path = format!("data/examples/{:02}.txt", day);

    #[cfg(not(debug_assertions))]
    let file_path = format!("data/{:02}.txt", day);

    let input: Vec<String> = read_inputs(&file_path).unwrap();
    println!("PART 1:");
    part1(&input);
    println!("PART 2:");
    part2(&input);
}
