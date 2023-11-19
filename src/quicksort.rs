pub(crate) fn sort_unstable_tuples(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: Vec<(i32, i32)> = Vec::new();
    for (pos, e) in nums.iter().enumerate() {
        elements.push((*e, pos as i32));
    }
    elements.sort_unstable_by_key(|k| k.0);
    return find_values_tuple(elements, target);
}

pub(crate) fn sort_tuples(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: Vec<(i32, i32)> = Vec::new();
    for (pos, e) in nums.iter().enumerate() {
        elements.push((*e, pos as i32));
    }
    elements.sort_by_key(|k| k.0);
    return find_values_tuple(elements, target);
}

fn find_values_tuple(elements: Vec<(i32, i32)>, target: i32) -> Vec<i32> {
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

pub(crate) fn sort_stable(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements = nums.clone();
    elements.sort();

    let targets = two_sum_sort_find_values(&elements, target);
    return two_sum_sort_search_for_original_indices(&nums, targets);
}

pub(crate) fn sort_unstable(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements = nums.clone();
    elements.sort_unstable();

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



pub(crate) fn sort_unstable_packed(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut elements: Vec<u64> = Vec::with_capacity(nums.len());
    for idx in 0..nums.len() {
        let mut combined = (nums[idx] as u64) << 32u64;
        combined |= idx as u64;
        elements.push(combined);
    }

    elements.sort_unstable();

    return two_sum_sort_u64_find_values(elements, target);
}

fn two_sum_sort_u64_find_values(elements: Vec<u64>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = elements.len() - 1;
    while i < j {
        let temp = ((elements[i] & 0xFFFFFFFF00000000) >> 32) as i32
            + ((elements[j] & 0xFFFFFFFF00000000) >> 32) as i32;

        if temp < target {
            i = i + 1;
        } else if temp > target {
            j = j - 1;
        } else {
            return vec![elements[i] as i32, elements[j] as i32];
        }
    }

    return vec![-1, -1];
}
