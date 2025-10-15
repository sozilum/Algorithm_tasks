impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
    
    let mut sec_num = String::new();
    for num in x.to_string().chars().rev(){
        sec_num.push(num);
    }

    if sec_num == x.to_string(){
        return true;
    } else{
        return false;
    };
    }
}


// The reference solution
impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
         if x < 0 {
        return false;
    }
    if x == 0 {
        return true;
    }
    let mut digits = vec![];
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    let front = digits.iter().take(digits.len() / 2);
    let back = digits.iter().rev().take(digits.len() / 2);
    front.eq(back)
    }
}