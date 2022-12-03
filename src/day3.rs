use itertools::Itertools;

pub fn run(input: String) {
    println!("Pt1: {}", pt1(&input));
    println!("Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let input_split = input.lines();
    let mut sum_pt1 = 0;
    for l in input_split {
        let first = &l[..l.len() / 2].chars().sorted().collect::<String>();
        let second = &l[l.len() / 2..].chars().sorted().collect::<String>();
        for a in first.chars() {
            match second.chars().position(|b| b == a) {
                None => continue,
                _ => {
                    sum_pt1 += add(a);
                    break;
                }
            }
        }
    }
    return sum_pt1;
}

fn pt2(input: &String) -> i32 {
    let mut input_split = input.lines();
    let mut total = 0;
    loop {
        let line;
        match input_split.next() {
            None => break,
            value => line = value.unwrap(),
        }
        let line2 = input_split.next().unwrap();
        let line3 = input_split.next().unwrap();
        let lines = [line, line2, line3];

        for a in get_chars() {
            let mut count = 0;
            for l in lines {
                match l.chars().position(|b| b == a) {
                    None => continue,
                    _ => {
                        count += 1;
                    }
                }
            }
            if count == 3 {
                total += add(a);
                break;
            }
        }
    }
    return total;
}

fn add(a: char) -> i32 {
    let chars = get_chars();
    return chars.iter().position(|&c| c == a).unwrap() as i32 + 1;
}

fn get_chars() -> [char; 52] {
    return [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
        'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
}
