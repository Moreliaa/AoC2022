pub fn run(input: String) {
    let input_split = input.lines();
    let mut score = 0;
    let mut score_pt2 = 0;
    for l in input_split {
        let a = l.chars().nth(0).unwrap();
        let b = l.chars().nth(2).unwrap();
        score += add_score(a, b);
        score_pt2 += add_score_pt2(a, b);
    }
    println!("Pt1: {}", score);
    println!("Pt2: {}", score_pt2);
}

fn add_score(a: char, b: char) -> i32 {
    let mut score = 0;
    if ((a == 'A' && b == 'Y') || (a == 'B' && b == 'Z') ||(a == 'C' && b == 'X')) { // win
        score += 6;
    } else if ((a == 'A' && b == 'Z') || (a == 'B' && b == 'X') ||(a == 'C' && b == 'Y')) { // loss

    } else { // draw
        score += 3;
    }
    
    if (b == 'X') {
        score += 1;
    } else if (b == 'Y') {
        score += 2;
    } else { // Z
        score += 3;
    }

    return score;
}

fn add_score_pt2(a: char, b: char) -> i32 {
    let mut score = 0;
    if (b == 'X') {
        score += 0;
    } else if (b == 'Y') {
        score += 3;
    } else { // Z
        score += 6;
    }
    if (a == 'A') {
        if (b == 'X') {
            score += 3;
        } else if (b == 'Y') {
            score += 1;
        } else { // Z
            score += 2;
        }
    } else if (a == 'B') {
        if (b == 'X') {
            score += 1;
        } else if (b == 'Y') {
            score += 2;
        } else { // Z
            score += 3;
        }
    } else { // Z
        if (b == 'X') {
            score += 2;
        } else if (b == 'Y') {
            score += 3;
        } else { // Z
            score += 1;
        }
    }
    return score;
}