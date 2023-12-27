#[derive(Debug, PartialEq, Eq, Clone)]
struct Task {
    id: usize,
    duration: usize,
    period: usize,
    turn_count: usize,
}

fn main() {
    println!("Algoritm Starts!!");

    let mut first = Task {
        id: 1,
        duration: 2,
        period: 10,
        turn_count: 0,
    };
    let mut second = Task {
        id: 2,
        duration: 3,
        period: 15,
        turn_count: 0,
    };
    let mut third = Task {
        id: 3,
        duration: 3,
        period: 10,
        turn_count: 0,
    };
    let mut fourth = Task {
        id: 4,
        duration: 4,
        period: 30,
        turn_count: 0,
    };

    let mut arr = vec![&mut first, &mut second, &mut third, &mut fourth];
    // rate_monotonic(&mut arr);
    earliest_deadline(&mut arr)

    // calc_lcm(&mut nums);
}

// fn rate_monotonic(arr: &mut Vec<&Task>) {
//     arr.sort_by_key(|task| task.duration);
//
//     for task in arr {
//         println!("{:?}", task);
//         println!("Task {} starts...", task.id);
//
//         sleep(Duration::from_secs(task.duration as u64));
//         println!("Task ends!");
//     }
// }

fn earliest_deadline(arr: &mut Vec<&mut Task>) {
    println!("Earliest Deadline.....");
    let mut periods: Vec<usize> = arr.iter().map(|x| x.period).collect();
    let lcm = calc_lcm(&mut periods).pop().unwrap();
    println!("lcm : {}", lcm);
    arr.sort_by_key(|x| x.period);
    println!("sorted array : {:?}", arr);
    let mut current_time = 0;
    let mut task_index = 0;
    // println!("{:?}", arr[0]);
    while current_time <= lcm {
        if task_index == arr.len() {
            task_index = 0;
        }
        let current_task = &mut arr[task_index];
        if current_time + current_task.duration > lcm {
            break;
        }
        if current_time < current_task.period * current_task.turn_count {
            current_time += 1;
            continue;
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
        task_index += 1;
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
