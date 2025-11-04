
fn main(){
	pub fn remove_dublicates(nums:&mut Vec<i32>) -> i32{
		let mut index_vector: Vec<i32> = vec![];
		let mut aux: i32 = nums[0];
		for (index, num) in &mut nums[1..].iter().enumerate(){
			match num{
				num if *num == aux => index_vector.push(index as i32),
				_ => aux = *num,
			}
		}

		for index_num in index_vector.iter().rev(){
			&mut nums.remove(*index_num as usize);
		}
	 	
		let answer = &mut nums.len().clone();
		*answer as i32
	}
}



//reference solution

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut set: HashSet<i32> = HashSet::<i32>::new();

        while j < nums.len() {
            match set.insert(nums[j]) {
                true => {
                    nums[i] = nums[j];
                    i += 1;
                    j += 1;
                }
                false => j += 1,
            };
        }
        return i as i32;
    }
}