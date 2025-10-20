pub fn longest_common_prefix(strs: Vec<String>) -> String{
	let mut aux: String = String::new();
	let mut answer: String = String::new();

	if strs.iter().len() != 1{

		for (word_index, word) in strs.iter().enumerate(){
			
			if aux != "" {

				for (letter_index, letters) in word.chars().enumerate(){
					if letters == aux.chars().nth(letter_index).unwrap(){
						answer.push(letters);
					}else{
						break;
					};
				}

				if word_index != strs.iter().len() - 1{
					aux = answer.clone();
					answer = "".to_string();

				}else{
					continue;
				};


			}else{
				aux = word.clone();
			};

		}
	}else{
		answer = strs.iter().next().unwrap().clone();
	};
		answer

}


fn main(){
	let strs: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
	let result = longest_common_prefix(strs);
	println!("результат {}", result);

	let strs:Vec<String> = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
	let result = longest_common_prefix(strs);
	println!("Ещё один тест {}", result);

	let strs:Vec<String> = vec!["a".to_string()];
	let result = longest_common_prefix(strs);
	println!("Ещё один тест {}", result);

	let strs:Vec<String> = vec!["aaa".to_string(),"aa".to_string(),"aaa".to_string()];
	let result = longest_common_prefix(strs);
	println!("Ещё один тест {}", result);

}
