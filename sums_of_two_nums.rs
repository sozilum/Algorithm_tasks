impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = Vec::new();
        let mut check: bool = false;
        for (first_index, first_num) in nums.iter().enumerate(){
            for (second_index, sec_num) in nums.iter().enumerate() {
                if target == first_num + sec_num && first_index != second_index{
                    answer.push(first_index.try_into().unwrap());
                    answer.push(second_index.try_into().unwrap());
                    check = true;
                }
            }
            if check == true{
                break;
            }
        }
        answer
    }
}

class Solution():
    def two_sum(nums: List, target: Int):    
    for (index, num) in nums.enumerate():
        for (sec_index, sec_num) in nums.enumerate():
            if first_index != sec_index and num + sec_num == target:
                return [index, sec_index] 



// The reference solution
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap: HashMap<i32, i32> = HashMap::new();
    for (index, number) in nums.iter().enumerate() {
        if hmap.contains_key(number) {
	    return vec![hmap[number], index as i32];
        }
        hmap.insert(target - *number, index as i32);
    }
    panic!("unprocessable");
    }
}