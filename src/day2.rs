pub fn run(input: String) {
    let input_split = input.lines();
    let mut score = (0, 0);
    for l in input_split {
        let a = l.chars().nth(0).unwrap();
        let b = l.chars().nth(2).unwrap();
        score = add_score(a, b, score.0, score.1);
    }
    println!("Pt1: {}", score.0);
    println!("Pt2: {}", score.1);
}

fn add_score(a: char, b: char, mut score_pt1: i32, mut score_pt2:i32) -> (i32, i32) {
    let chars_a = ['A', 'B', 'C'];
    let chars_b = ['Y', 'Z', 'X'];
    let pos_a = chars_a.iter().position(|&c| c == a).unwrap();
    let pos_b = chars_b.iter().position(|&c| c == b).unwrap();
    score_pt1 += ((pos_b as i32 + 1) % 3) + 1;
    if pos_b == pos_a { // win
        score_pt1 += 6;
    } else if pos_a == (pos_b + 1) % 3 { // draw
        score_pt1 += 3;
    }

    score_pt2 += ((pos_b as i32 + 1) % 3) * 3;
    score_pt2 += ((pos_a as i32 + pos_b as i32) % 3) + 1; 

    return (score_pt1, score_pt2);
}