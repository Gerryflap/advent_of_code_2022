use std::collections::LinkedList;
use std::iter;

use crate::aoc_day;

pub struct Day07;

impl aoc_day::AocDay for Day07 {
    fn run(&self, inp: &str) -> String {
        format!("q1: {}\nq2: {}", q1(inp), q2(inp))
    }

    fn get_path(&self) -> &str {
        "day_07"
    }
}

fn q1(inp: &str) -> u64 {
    q1_total(&parse_into_file_tree(inp))
}

fn q2(inp: &str) -> u64 {
    let dir = &parse_into_file_tree(inp);
    let remaining_size = 70000000 - get_total_size(dir);
    q2_find_size_to_delete(dir, 30000000 - remaining_size).unwrap()
}

#[derive(Clone)]
struct Dir {
    name: String,
    children: Vec<Dir>,
    files: Vec<File>,
}

#[derive(Clone)]
struct File {
    name: String,
    size: u64,
}

fn parse_into_file_tree(input: &str) -> Dir {
    let mut lines: LinkedList<String> = input.lines().map(|s| s.to_string()).collect();
    println!("{:?}", lines);
    let mut path: LinkedList<String> = LinkedList::new();    
    let mut file_system: Dir = Dir {
        name: "".to_string(),
        children: vec!(),
        files: vec!(),
    };

    while !lines.is_empty() {
        let line: String = lines.pop_front().unwrap();
        if line == "$ cd .." {
            path.pop_front();
        } else if line.starts_with("$ cd ") {
            let dir_name: &String = &line.to_string()[5..].to_string();
            let old_path = path.clone();
            path.push_front(dir_name.clone());

            if !is_dir(&path, &file_system) {
                file_system = mkdir(dir_name.clone(), &old_path, &file_system)
            }
    
        } else if line.starts_with("$ ls") {
            (lines, file_system) = parse_ls(lines, &file_system, &path);
        }
    }
    file_system
}

fn parse_ls(lines: LinkedList<String>, file_system: &Dir, current_path: &LinkedList<String>) -> (LinkedList<String>, Dir) {
    let new_lines = &mut lines.clone();
    let mut fs: Dir = file_system.clone();
    while ! (new_lines.front().is_none() ||  new_lines.front().unwrap().starts_with("$")) {
        let line: String = new_lines.pop_front().unwrap();
        if line.starts_with("dir ") {
            let dir_name: String = line.to_string()[4..].to_string();
            fs = mkdir(dir_name, current_path, &fs);
        } else {
            println!("line: {}", line);
            let split = &mut line.split(" ");
            let dig = &split.next().unwrap().to_string();
            println!("{}", dig);
            let file = File {
                size: dig.parse().unwrap(),
                name: split.next().unwrap().to_string()
            };
            fs = touch(file, current_path, &fs)
        }

    }
    (new_lines.clone(), fs)

}

fn is_dir(path: &LinkedList<String>, dir: &Dir) -> bool {
    if path.is_empty() {
        true
    } else {
        let next_path = &mut path.clone();
        let next_dir_name = next_path.pop_back().unwrap();
        let next_dir: Option<Dir> = dir.children.iter().find(|dir| dir.name == next_dir_name).cloned();
        next_dir.map(|nd| is_dir(next_path, &nd)).unwrap_or(false)
    }
}



fn mkdir(name: String, path: &LinkedList<String>, root: &Dir) -> Dir {
    if path.is_empty() {
        let root_children = &mut root.children.clone();
        let dir: Dir = Dir {
                                name: name,
                                children: vec!(),
                                files: vec!(),
                            };
        root_children.push(dir);
        Dir {
            children: root_children.clone(),
            ..root.clone()
        }
    } else {
        let reduced_path: &mut LinkedList<String> = &mut path.clone();
        let next_dir: String = reduced_path.pop_back().unwrap();
        Dir {
            children: root.children.iter().map(|dir: &Dir| {
                if dir.name == next_dir {
                    mkdir(name.clone(), reduced_path, dir)
                } else {
                    dir.clone()
                }
            }).collect(),
            ..root.clone()
        }
    }
}

fn touch(file: File, path: &LinkedList<String>, root: &Dir) -> Dir {
    if path.is_empty() {
        let root_files = &mut root.files.clone();
        root_files.push(file);
        Dir {
            files: root_files.clone(),
            ..root.clone()
        }
    } else {
        let reduced_path: &mut LinkedList<String> = &mut path.clone();
        let next_dir: String = reduced_path.pop_back().unwrap();
        Dir {
            children: root.children.iter().map(|dir: &Dir| {
                if dir.name == next_dir {
                    touch(file.clone(), reduced_path, dir)
                } else {
                    dir.clone()
                }
            }).collect(),
            ..root.clone()
        }
    }
}

fn get_total_size(dir: &Dir) -> u64 {
    dir.children.iter().map(get_total_size).sum::<u64>() + dir.files.iter().map(|f| f.size).sum::<u64>()
}

fn q1_total(dir: &Dir) -> u64 {
    let own_size = Some(get_total_size(dir)).filter(|tot| *tot <= 100000 as u64).unwrap_or(0);
    own_size + dir.children.iter().map(q1_total).sum::<u64>()
}

fn q2_find_size_to_delete(dir: &Dir, min_size: u64) -> Option<u64> {
    let own_size = Some(dir).map(get_total_size).filter(|tot| *tot >= min_size);
    let res = dir.children.iter().map(|d| q2_find_size_to_delete(d, min_size))
        .chain(iter::once(own_size)).filter_map(|o| o).min();
    println!("{}: {:?}, {:?}", dir.name, res, own_size);
    res
}