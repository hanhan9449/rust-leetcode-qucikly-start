use std::collections::HashMap;
use std::unreachable;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![-1, -1]
}

fn two_sum1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut kv_map = HashMap::new();
    for i in 0..nums.len() {
        match kv_map.get(&(target - nums[i])) {
            Some(&index) => {
                return vec![index as i32, i as i32];
            }
            None => {
                kv_map.insert(nums[i], i);
            }
        }
    }
    unreachable!()
}

#[test]
fn test_1() {
    assert_eq!(two_sum(vec!(1, 2, 3, 4), 7), vec!(2, 3));
    assert_eq!(two_sum1(vec!(1, 2, 3, 4), 7), vec!(2, 3));
}