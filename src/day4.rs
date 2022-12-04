pub fn run(input: String) {
    let mut count_pt1 = 0;
    let mut count_pt2 = 0;
    for l in input.lines() {
        let line_split = l.split(',');
        let mut min_max = [0; 4];
        for (idx, part) in line_split.enumerate() {
            let part_split = part.split('-');
            for (sub_idx, sub_part) in part_split.enumerate() {
                min_max[sub_idx + (idx * 2)] = sub_part.parse::<i32>().unwrap();
            }
        }
        
        if (min_max[0] >= min_max[2] && min_max[1] <= min_max[3])
            || (min_max[0] <= min_max[2] && min_max[1] >= min_max[3])
        {
            count_pt1 += 1;
        }

        if (min_max[0] < min_max[2] && min_max[1] >= min_max[2])
            || (min_max[0] <= min_max[3] && min_max[1] > min_max[3])
            || (min_max[0] >= min_max[2] && min_max[1] <= min_max[3])
            || (min_max[0] <= min_max[2] && min_max[1] >= min_max[3])
        {
            count_pt2 += 1;
        }
    }

    println!("Pt1: {}", count_pt1);
    println!("Pt2: {}", count_pt2);
}
