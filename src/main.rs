use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::time::SystemTime;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(1234569);

    println!("generating");
    let nums: Vec<i32> = (0..10000).map(|_| rng.gen_range(0..10000)).collect();
    let search_functions: Vec<(&str, &dyn Fn(Vec<i32>, i32) -> Vec<i32>)> = vec![
        ("sort_stable", &two_sum_sort_stable),
        ("sort_unstable", &two_sum_sort_unstable),
        ("hash", &two_sum_hash),
        ("naive", &two_sum_naive),
    ];

    let search_limit_in_secs: u64 = 10;
    for f in &search_functions {
        let mut run_times: Vec<u128> = Vec::new();
        println!("Function {} start", f.0);
        let start_search = SystemTime::now();
        let mut search_duration: u64 = 0;
        while search_duration < search_limit_in_secs {
            let run_nums = nums.clone();
            let start_iteration = SystemTime::now();

            (f.1)(run_nums, -1);

            let end = SystemTime::now();
            let iteration_duration = end.duration_since(start_iteration).unwrap().as_nanos();

            print!(".");
            io::stdout().flush().unwrap();
            run_times.push(iteration_duration);

            search_duration = end.duration_since(start_search).unwrap().as_secs();
        }

        let sum = run_times.iter().sum::<u128>();
        let count = run_times.len() as u128;
        let mean = sum / count;
        println!();
        println!("Mean of run times (nanos): {}", mean);
    }
}

fn two_sum_sort_stable(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements = nums.clone();
    elements.sort();
    let targets = two_sum_sort_find_values(elements, target);
    return two_sum_sort_search_for_indices(nums, targets);
}

fn two_sum_sort_unstable(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements = nums.clone();
    elements.sort_unstable();
    let targets = two_sum_sort_find_values(elements, target);
    return two_sum_sort_search_for_indices(nums, targets);
}

fn two_sum_sort_search_for_indices(nums: Vec<i32>, targets: (i32, i32)) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(2);
    let mut i = 0;
    while i < nums.len() && result.len() < 2 {
        let num = nums[1];
        if targets.0 == num {
            result.push(num);
        } else if targets.1 == num {
            result.push(num);
        }

        i = i + 1;
    }

    return result;
}

fn two_sum_sort_find_values(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        let temp = nums[i] + nums[j];
        if temp < target {
            i = i + 1;
        } else if temp > target {
            j = j - 1;
        } else {
            return (nums[i], nums[j]);
        }
    }

    return (-1, -1);
}

fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match complements.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => complements.insert(target - num, i as i32),
        };
    }
    return Vec::new();
}

fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return Vec::new();
}
