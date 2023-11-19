use voracious_radix_sort::RadixSort;

pub(crate) fn sort_radix_1t(nums: Vec<i32>, target: i32) -> Vec<i32> {
    two_sum_radix_ints(1)(nums, target)
}

pub(crate) fn sort_radix_4t(nums: Vec<i32>, target: i32) -> Vec<i32> {
    two_sum_radix_ints(4)(nums, target)
}

pub(crate) fn sort_radix_8t(nums: Vec<i32>, target: i32) -> Vec<i32> {
    two_sum_radix_ints(8)(nums, target)
}

fn two_sum_radix_ints(thread_count: usize) -> impl Fn(Vec<i32>, i32) -> Vec<i32> {
    move |nums, target| {
        let mut elements: Vec<i32> = nums.clone();
        if thread_count == 1 {
            elements.voracious_sort();
        } else {
            elements.voracious_mt_sort(thread_count);
        }

        return two_sum_sort_find_values(&elements, target);
    }
}

fn two_sum_sort_find_values(nums: &[i32], target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = nums.len() - 1;
    while i < j {
        let temp = nums[i] + nums[j];
        if temp < target {
            i += 1;
        } else if temp > target {
            j -= 1;
        } else {
            return vec![nums[i], nums[j]];
        }
    }

    vec![-1, -1]
}
