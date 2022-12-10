use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct FileItem {
    size: u64,
    parent: Option<String>,
    children: Vec<String>,
}

fn main() {
    let input = include_str!("../input-test.txt").trim();

    let root = parse_input(input);
    let mut directory_list = HashMap::new();
    for (key, value) in root.iter() {
        if value.size == 0 {
            let directory_size = calculate_directory_size(&root, key);
            directory_list.insert(key, directory_size);
        }
    }

    let directories = get_directories(&root);
    for dir in directories.clone() {
        let dir_path = root.get(&dir).unwrap();
        for child in &dir_path.children {
            if directories.contains(&child) {
                println!("{:?}", dir_path.children);
                println!("{}", child);
            }
        }
    }

    for (key, value) in directory_list.iter() {
        let mut total = value.to_owned();
        for (key1, value1) in directory_list.iter() {
            if key == key1 {
                continue;
            }

            if value + value1 > 100000 {
                println!("{total}");
                break;
            } else {
                total += value1.to_owned();
            }
        }
    }
}

fn get_directories(file_list: &HashMap<String, FileItem>) -> Vec<String> {
    let mut file_items = vec![];

    for (key, value) in file_list.iter() {
        if value.size == 0 {
            file_items.push(key.to_string())
        }
    }

    file_items
}

fn calculate_directory_size(file_list: &HashMap<String, FileItem>, key: &str) -> u64 {
    let mut total = 0;

    if let Some(dir) = file_list.get(key) {
        for child in dir.children.clone() {
            let (_child_key, child_obj) = file_list.get_key_value(&child).unwrap();

            if child_obj.size != 0 {
                total += child_obj.size;
            }
        }
    }

    total
}

fn parse_input(input: &str) -> HashMap<String, FileItem> {
    let mut file_list: HashMap<String, FileItem> = HashMap::new();

    let mut current: String = "/".to_string();
    let mut path_history: Vec<String> = Vec::new();
    for line in input.lines() {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line[0] == "$" && split_line[1] == "cd" {
            let to_dir = split_line[2];
            if to_dir == current {
                continue;
            } else if to_dir == ".." {
                if current != "/" {
                    current = path_history.pop().unwrap();
                }
            } else {
                path_history.push(current.to_string());
                current = to_dir.to_string();
            }
        } else if split_line[0] != "$" {
            let child_name = split_line[1].to_string();
            let child_size: u64 = split_line[0].parse().unwrap_or(0);

            let mut parent: Option<String> = None;
            if current != "/" {
                let history_length = path_history.len();
                parent = Some(path_history[history_length - 1].clone());
            }

            file_list.entry(child_name.clone()).or_insert(FileItem {
                size: child_size,
                parent: Some(current.clone()),
                children: vec![],
            });

            file_list
                .entry(current.clone())
                .and_modify(|fi| fi.children.push(child_name.to_string()))
                .or_insert(FileItem {
                    size: child_size,
                    parent,
                    children: vec![child_name],
                });
        }
    }

    file_list
}
