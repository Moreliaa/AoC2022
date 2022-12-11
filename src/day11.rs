use std::collections::VecDeque;
struct Monkey {
    inspections: i128,
    items: VecDeque<i128>,
    test_val: i128,
    true_val: i128,
    false_val: i128,
}

pub fn run(_: String) {
    let mut monkeys = [
        Monkey {
            // 0
            inspections: 0,
            items: VecDeque::from([83, 88, 96, 79, 86, 88, 70]),
            test_val: 11,
            true_val: 2,
            false_val: 3,
        },
        Monkey {
            // 1
            inspections: 0,
            items: VecDeque::from([59, 63, 98, 85, 68, 72]),
            test_val: 5,
            true_val: 4,
            false_val: 0,
        },
        Monkey {
            // 2
            inspections: 0,
            items: VecDeque::from([90, 79, 97, 52, 90, 94, 71, 70]),
            test_val: 19,
            true_val: 5,
            false_val: 6,
        },
        Monkey {
            // 3
            inspections: 0,
            items: VecDeque::from([97, 55, 62]),
            test_val: 13,
            true_val: 2,
            false_val: 6,
        },
        Monkey {
            // 4
            inspections: 0,
            items: VecDeque::from([74, 54, 94, 76]),
            test_val: 7,
            true_val: 0,
            false_val: 3,
        },
        Monkey {
            // 5
            inspections: 0,
            items: VecDeque::from([58]),
            test_val: 17,
            true_val: 7,
            false_val: 1,
        },
        Monkey {
            // 6
            inspections: 0,
            items: VecDeque::from([66, 63]),
            test_val: 2,
            true_val: 7,
            false_val: 5,
        },
        Monkey {
            // 7
            inspections: 0,
            items: VecDeque::from([56, 56, 90, 96, 68]),
            test_val: 3,
            true_val: 4,
            false_val: 1,
        },
    ];

    let kgv: i128 = 9699690;
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while monkeys[idx].items.len() > 0 {
                monkeys[idx].inspections += 1;
                let mut next = monkeys[idx].items.pop_front().unwrap();

                if idx == 0 {
                    next = op0(next)
                } else if idx == 1 {
                    next = op1(next)
                } else if idx == 2 {
                    next = op2(next)
                } else if idx == 3 {
                    next = op3(next)
                } else if idx == 4 {
                    next = op4(next)
                } else if idx == 5 {
                    next = op5(next)
                } else if idx == 6 {
                    next = op6(next)
                } else if idx == 7 {
                    next = op7(next)
                }
                //next = next / 3; // uses inherent rounding down with i128
                next = next % kgv;
                if next % monkeys[idx].test_val == 0 {
                    monkeys[monkeys[idx].true_val as usize]
                        .items
                        .push_back(next);
                } else {
                    monkeys[monkeys[idx].false_val as usize]
                        .items
                        .push_back(next);
                }
            }
        }
    }

    for idx in 0..monkeys.len() {
        println!("{} {}", idx, monkeys[idx].inspections);
    }
}

fn op0(next: i128) -> i128 {
    next * 5
}
fn op1(next: i128) -> i128 {
    next * 11
}
fn op2(next: i128) -> i128 {
    next + 2
}
fn op3(next: i128) -> i128 {
    next + 5
}
fn op4(next: i128) -> i128 {
    next * next
}
fn op5(next: i128) -> i128 {
    next + 4
}
fn op6(next: i128) -> i128 {
    next + 6
}
fn op7(next: i128) -> i128 {
    next + 7
}
