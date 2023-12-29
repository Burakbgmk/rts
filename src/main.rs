#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Task {
    id: usize,
    duration: usize,
    period: usize,
    turn_count: usize,
}

fn main() {
    let mut test1 = vec![
        Task {
            id: 1,
            duration: 1,
            period: 4,
            turn_count: 0,
        },
        Task {
            id: 2,
            duration: 2,
            period: 5,
            turn_count: 0,
        },
        Task {
            id: 3,
            duration: 5,
            period: 20,
            turn_count: 0,
        },
    ];

    let mut test2 = vec![
        Task {
            id: 1,
            duration: 2,
            period: 5,
            turn_count: 0,
        },
        Task {
            id: 2,
            duration: 4,
            period: 7,
            turn_count: 0,
        },
    ];

    let mut test3 = vec![
        Task {
            id: 1,
            duration: 2,
            period: 4,
            turn_count: 0,
        },
        Task {
            id: 2,
            duration: 3,
            period: 8,
            turn_count: 0,
        },
        Task {
            id: 3,
            duration: 2,
            period: 16,
            turn_count: 0,
        },
    ];

    println!("For Test 1.....");
    test1.sort_by_key(|x| x.duration);
    execute_algoritm("Rate Monotonic", &mut test1.clone());
    test1.sort_by_key(|x| x.period);
    execute_algoritm("Earliest Deadline", &mut test1);

    println!("For Test 2.....");
    test2.sort_by_key(|x| x.duration);
    execute_algoritm("Rate Monotonic", &mut test2.clone());
    test2.sort_by_key(|x| x.period);
    execute_algoritm("Earliest Deadline", &mut test2);

    println!("For Test 3.....");
    test3.sort_by_key(|x| x.duration);
    execute_algoritm("Rate Monotonic", &mut test3.clone());
    test3.sort_by_key(|x| x.period);
    execute_algoritm("Earliest Deadline", &mut test3);
}

fn execute_algoritm(algoritm_name: &str, arr: &mut Vec<Task>) {
    println!("{}....", algoritm_name);
    let mut periods: Vec<usize> = arr.iter().map(|x| x.period).collect();
    let lcm = calc_lcm(&mut periods).pop().unwrap();
    arr.sort_by_key(|x| x.period);
    let mut current_time = 0;
    let mut current_count = 0;
    while current_time <= lcm {
        let mut task_index: usize;
        let current_task_index = arr
            .iter()
            .position(|x| x.turn_count == current_count && current_time >= x.period * x.turn_count);
        task_index = current_task_index.unwrap_or_default();
        if current_task_index == None {
            let mut is_legit = false;
            while task_index < arr.len() {
                if current_time >= arr[task_index].period * arr[task_index].turn_count {
                    is_legit = true;
                    break;
                }
                task_index += 1;
            }
            if !is_legit {
                current_time += 1;
                continue;
            }
        }
        let current_task = &mut arr[task_index];
        if current_time + current_task.duration > lcm {
            break;
        }
        current_task.turn_count += 1;
        println!(
            "Task {}/ {}-{} / {}. turn",
            current_task.id,
            current_time,
            current_time + current_task.duration,
            current_task.turn_count
        );
        current_time += current_task.duration;
        // println!("end at {}", current_time);
        if !arr.iter().any(|x| x.turn_count == current_count) {
            current_count += 1;
            // println!(
            //     "Count increased to {} at time {}",
            //     current_count, current_time
            // );
        }
    }
    println!("Finished....");
}

fn calc_lcm(nums: &mut Vec<usize>) -> &mut Vec<usize> {
    let max = usize::MAX;
    nums.sort();
    let greatest = nums.pop().unwrap();
    // println!("{}", greatest);
    nums.reverse();
    let smallest = nums.pop().unwrap();
    // println!("{}", smallest);
    for i in greatest..max {
        // println!("i : {}", i);
        if i % smallest == 0 && i % greatest == 0 {
            nums.push(i);
            break;
        }
    }
    // println!("{:?}", nums);
    if nums.len() > 1 {
        calc_lcm(nums);
    }
    return nums;
}
