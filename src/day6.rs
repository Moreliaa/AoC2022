use std::collections::VecDeque;

pub fn run(input: String) {
    let mut queue: VecDeque<char> = VecDeque::new();
    let mut pt1_done = false;
    for (i, c) in input.chars().enumerate() {
        let max_size = match pt1_done {
            true => 14,
            false => 4,
        };
        queue = put_in_queue(c, queue, max_size);
        if queue.len() == max_size {
            println!("Pt{}: {}", match pt1_done { true => 2, false => 1 }, i + 1);
            pt1_done = true;
        }
    }
}

fn put_in_queue(c: char, mut queue: VecDeque<char>, size: usize) -> VecDeque<char> {
    while queue.contains(&c) {
        queue.pop_front();
    }
    queue.push_back(c);
    if queue.len() > size {
        queue.pop_front();
    }
    return queue;
}