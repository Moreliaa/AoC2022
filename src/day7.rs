use std::vec::Vec;
use std::collections::HashMap;
struct Directory {
    parent_directory: i32,
    name: String,
    size: i32,
    files: Vec<File>,
    directories: Vec<i32>,
}

struct File {
    name: String,
    size: i32
}


pub fn run(input: String) {
    let mut directories: HashMap<i32, Directory> = HashMap::new();
    let mut next_id = 0;
    let root = Directory {
        parent_directory: next_id,
        name: String::from("/"),
        size: 0,
        files: Vec::new(),
        directories: Vec::new(),
    };
    directories.insert(next_id, root);
    next_id += 1;

    let mut current_dir_id = 0;
    let input_split = input.lines();
    let mut cmd = "";
    let mut ls_params:Vec<&str> = Vec::new();

    for line in input_split {
        let mut line_split = line.split(' ');
        if line_split.nth(0).unwrap() == "$" {
            if cmd == "ls" && ls_params.len() > 0 {
                (next_id, directories) = cmd_ls(directories, next_id, current_dir_id, &ls_params);
                ls_params = Vec::new();
            }
            cmd = line_split.nth(0).unwrap();
            if cmd == "cd" {
                current_dir_id = cmd_cd(&directories, current_dir_id, line_split.nth(0).unwrap());
            } else if cmd == "ls" {
                // nothing
            } else {
                panic!("no cmd");
            }
        } else if cmd == "ls" {
            ls_params.push(line);
        }
    }
    if cmd == "ls" && ls_params.len() > 0 {
        (_, directories) = cmd_ls(directories, next_id, current_dir_id, &ls_params);
    }

    let (pt1, total, dir_sizes) = calc_directory_size(&directories, 0);
    println!("Pt1: {}", pt1);
    let min_value_to_delete = 30000000 - (70000000 - total);
    let mut min_dir_size = total;
    for value in String::from(dir_sizes).split(",") {
        let value_as_int = value.parse::<i32>().unwrap();
        if value_as_int < min_dir_size && value_as_int >= min_value_to_delete {
            min_dir_size = value_as_int;
        }
    }
    println!("Pt2: {}", min_dir_size);
}

fn cmd_cd (map: &HashMap<i32, Directory>, current_dir_id: i32, param: &str) -> i32 {
    if param == "/" {
        return 0;
    } else if param == ".." {
        return map.get(&current_dir_id).unwrap().parent_directory;
    } else {
        for dir in map.get(&current_dir_id).unwrap().directories.iter() {
            let dir_map = map.get(&dir).unwrap();
            if dir_map.name == param {
                return *dir;
            }
        }
    }
    panic!("cmd_cd");
}

fn cmd_ls (mut map: HashMap<i32, Directory>, mut next_id: i32, current_dir_id: i32, params: &Vec<&str>) -> (i32, HashMap<i32, Directory>) {
    {
        let dir_from_map = map.get_mut(&current_dir_id).unwrap();
        if dir_from_map.files.len() > 0 || dir_from_map.directories.len() > 0 {
            return (next_id, map);
        }
    }
    for p in params {
        let mut slices = p.split(' ');
        let first = slices.nth(0).unwrap();
        let second = slices.nth(0).unwrap();
        if first == "dir" {
            let root = Directory {
                parent_directory: current_dir_id,
                name: String::from(second),
                size: 0,
                files: Vec::new(),
                directories: Vec::new(),
            };
            map.insert(next_id, root);
            {
                let dir_from_map = map.get_mut(&current_dir_id).unwrap();
                dir_from_map.directories.push(next_id);
                next_id += 1;
            }
        } else { // file
            let file = File {
                name: String::from(second),
                size: first.parse::<i32>().unwrap(),
            };
            {
                let dir_from_map = map.get_mut(&current_dir_id).unwrap();
                dir_from_map.files.push(file);
            }
            
        }
    }
    return (next_id, map);
}

fn calc_directory_size (map: &HashMap<i32, Directory>, current_dir: i32) -> (i32, i32, String) {
    let mut result_pt1 = 0;
    let mut total = 0;
    let dir = map.get(&current_dir).unwrap();
    for file in dir.files.iter() {
        total += file.size;
    }
    let mut dir_sizes = String::from("");
    for sub_dir in dir.directories.iter() {
        let (sub_result_pt1, sub_result, sub_dir_sizes) = calc_directory_size(map, *sub_dir);
        result_pt1 += sub_result_pt1;
        total += sub_result;
        if dir_sizes != "" {
            dir_sizes = dir_sizes + ",";
        }
        dir_sizes = dir_sizes + &sub_dir_sizes;
    }

    if total <= 100000 {
        result_pt1 += total;
    }
    if dir_sizes != "" {
        dir_sizes = dir_sizes + ",";
    }
    dir_sizes = dir_sizes + &total.to_string();
    return (result_pt1, total, dir_sizes);
}