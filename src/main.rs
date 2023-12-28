#[derive(Debug, PartialEq, Eq, Clone, Default)]
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

    let arr = vec![&mut first, &mut second, &mut third, &mut fourth];
    // rate_monotonic(&mut arr);
    earliest_deadline(arr)

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

fn earliest_deadline(mut arr: Vec<&mut Task>) {
    println!("Earliest Deadline.....");
    let mut periods: Vec<usize> = arr.iter().map(|x| x.period).collect();
    let lcm = calc_lcm(&mut periods).pop().unwrap();
    println!("lcm : {}", lcm);
    arr.sort_by_key(|x| x.period);
    println!("sorted array : {:?}", arr);
    let mut current_time = 0;
    let mut current_count = 0;
    while current_time <= lcm {
        let mut task_index = 0;
        let current_task_index = arr
            .iter()
            .position(|x| x.turn_count == current_count && current_time >= x.period * x.turn_count);
        task_index = current_task_index.unwrap_or_default();
        if current_task_index == None {
            let mut is_legit = false;
            while task_index < arr.len() {
                // println!(
                //     "task index : {}, current count : {}, current time {}",
                //     task_index, current_count, current_time
                // );
                if current_time >= arr[task_index].period * arr[task_index].turn_count {
                    is_legit = true;
                    // println!("legittt!");
                    break;
                }
                task_index += 1;
            }
            if !is_legit {
                // println!("isnt legit!");
                current_time += 1;
                continue;
            }
        }
        let current_task = &mut arr[task_index];
        // println!("Turn is for {:?}", current_task);
        if current_time + current_task.duration > lcm {
            println!("duration: {}", current_task.duration);
            println!("end at {}", current_time);
            println!("Breaks due to duration exceeds max limit");
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
        println!("end at {}", current_time);
        if !arr.iter().any(|x| x.turn_count == current_count) {
            current_count += 1;
            println!(
                "Count increased to {} at time {}",
                current_count, current_time
            );
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
