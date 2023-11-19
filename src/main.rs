use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::HashMap;
use std::io::Write;
use std::time::{Duration, SystemTime};
use std::{io, vec};

use num_format::{Locale, ToFormattedString};

mod quicksort;
mod radix;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(1234569);

    println!("generating...");
    let nums: Vec<i32> = (0..1_000_000).map(|_| rng.gen_range(0..1000000)).collect();
    let search_functions: Vec<(&str, &dyn Fn(Vec<i32>, i32) -> Vec<i32>)> = vec![
        ("sort_stable_linear", &quicksort::sort_stable),
        ("sort_unstable_linear", &quicksort::sort_unstable),
        ("sort_unstable_packed", &quicksort::sort_unstable_packed),
        ("sort_radix_1t", &radix::sort_radix_1t),
        ("sort_radix_4t", &radix::sort_radix_4t),
        ("sort_radix_8t", &radix::sort_radix_8t),
        ("sort_tuples", &quicksort::sort_tuples),
        ("sort_unstable_tuples", &quicksort::sort_unstable_tuples),
        ("hash", &two_sum_hash),
        ("naive", &two_sum_naive),
    ];

    for f in &search_functions {
        let mut run_times: Vec<u128> = Vec::new();
        print!("{}", f.0);
        io::stdout().flush().unwrap();
        let start_search = SystemTime::now();
        let mut search_duration: Duration = Duration::from_nanos(0);
        while search_duration.as_secs() < 60 {
            let run_nums = nums.clone();
            let start_iteration = SystemTime::now();
            (f.1)(run_nums, -1);
            let end = SystemTime::now();
            let iteration_duration = end.duration_since(start_iteration).unwrap().as_nanos();
            run_times.push(iteration_duration);
            let prev = search_duration.as_secs();
            search_duration = end.duration_since(start_search).unwrap();
            if prev != search_duration.as_secs() {
                print!(".");
                io::stdout().flush().unwrap();
            }
        }

        let sum = run_times.iter().sum::<u128>();
        let count = run_times.len() as u128;
        let mean = sum / count;
        println!();
        println!(
            "  mean runtime {} nanos ({} iterations)",
            mean.to_formatted_string(&Locale::da),
            count.to_formatted_string(&Locale::da)
        );
    }
}

fn two_sum_sort_unstable_mem_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: Vec<(i32, i32)> = Vec::new();
    for (pos, e) in nums.iter().enumerate() {
        elements.push((*e, pos as i32));
    }
    elements.sort_unstable_by_key(|k| k.0);
    return two_sum_sort_mem_find_values(elements, target);
}

fn two_sum_sort_mem_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: Vec<(i32, i32)> = Vec::new();
    for (pos, e) in nums.iter().enumerate() {
        elements.push((*e, pos as i32));
    }
    elements.sort_by_key(|k| k.0);
    return two_sum_sort_mem_find_values(elements, target);
}

fn two_sum_sort_mem_find_values(elements: Vec<(i32, i32)>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = elements.len() - 1;
    while i < j {
        let temp = elements[i].0 + elements[j].0;
        if temp < target {
            i += 1;
        } else if temp > target {
            j -= 1;
        } else {
            return vec![elements[i].1, elements[j].1];
        }
    }

    return vec![-1, -1];
}

fn sort_stable_linear_search(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements = nums.clone();
    elements.sort();

    let targets = two_sum_sort_find_values(&elements, target);
    return two_sum_sort_search_for_original_indices(&nums, targets);
}

fn two_sum_sort_find_values(nums: &[i32], target: i32) -> (i32, i32) {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        let temp = nums[i] + nums[j];
        if temp < target {
            i += 1;
        } else if temp > target {
            j -= 1;
        } else {
            return (nums[i], nums[j]);
        }
    }

    return (-1, -1);
}

fn two_sum_sort_search_for_original_indices(nums: &[i32], targets: (i32, i32)) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::with_capacity(2);
    let mut i = 0;
    while i < nums.len() && result.len() < 2 {
        let num = nums[i];
        if targets.0 == num || targets.1 == num {
            result.push(i as i32);
        }

        i += 1;
    }

    return result;
}

fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complements: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        match complements.get(num) {
            Some(&index) => return vec![index, i as i32],
            None => complements.insert(target - num, i as i32),
        };
    }

    return vec![-1, -1];
}

fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return vec![-1, -1];
}
