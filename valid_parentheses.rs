pub fn valid_parenthese(s: String) -> bool{
	let char_vector: Vec<char> = s.chars().collect();
	let mut answer: bool = true;
	let mut check_vector: Vec<char> = vec![];
	let mut flag: bool = false;


	if s.len() >= 2{
		for symbol in char_vector.iter(){

			match symbol{
				symbol if *symbol == '('=>  check_vector.push(*symbol),
				symbol if *symbol == '{' => check_vector.push(*symbol),
				symbol if *symbol == '[' => check_vector.push(*symbol),
				
				symbol if *symbol == ')' =>{
					match check_vector.last(){
						Some(symbol) if *symbol == '(' => {
							check_vector.pop();
							flag = true;
						},
						None => {
							answer = false;
						},
						Some(_) => {
							answer = false;
						},
					};

				},
				symbol if *symbol == '}' =>{
					match check_vector.last(){
						Some(symbol) if *symbol == '{' => {
							check_vector.pop();
							flag = true;
						},
						None => {
							answer = false;
						},
						Some(_) => {
							answer = false;
						},
					};
				},
				symbol if *symbol == ']' =>{
					match check_vector.last(){
						Some(symbol) if *symbol == '[' => {
							check_vector.pop();
							flag = true;
						},
						None => {
							answer = false;
						},
						Some(_) => {
							answer = false;
						},
					};
				},
				_ => answer = false,
			};
		};


	}else{
		answer = false;
	};

	if check_vector.len() != 0{
		answer = false;
	};

	if flag == false{
		answer = false;
	};

	answer
}

//the reference solution

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut my_stack = Vec::with_capacity(s.len());
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                my_stack.push(c);
            } else {
                match my_stack.pop() {
                    None => {return false;},
                    Some(last) => {
                        match (last, c) {
                            ('(', ')') => {continue;},
                            ('[', ']') => {continue;},
                            ('{', '}') => {continue;},
                            (_, _) => {return false;}
                        }
                    }
                }
            }
        }
        my_stack.is_empty()
    }
}




fn main(){

	let s = "(]".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");

	let s = "()[]{}".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be true", answer);
	println!("");

	let s = "(]".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");

	let s = "([])".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be true", answer);
	println!("");

	let s = "([)])".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");

	let s = "([){]}".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");

	let s = "((".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");
	
	let s = "){".to_string();
	let answer = valid_parenthese(s);
	println!("answer {}, must be false", answer);
	println!("");

	let s = "([]){".to_string();
	let answer = valid_parenthese(s);
	println!("asnwer {}, must be false", answer);
	println!("");
}
