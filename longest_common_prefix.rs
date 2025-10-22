

pub fn longest_common_prefix(strs: Vec<String>) -> String{
	let mut aux: String = String::new();
	let mut answer: String = String::new();
	let mut flag: bool = false;
	
	match strs {
		ref strs_vec if strs_vec.iter().len() == 1 => answer = strs.iter().next().unwrap().clone(),
		ref strs_vec if strs_vec[0].chars().next() != strs_vec[1].chars().next() => answer = "".to_string(),
		_ =>
			for (word_index, word) in strs.iter().enumerate(){
				if aux != "" {
					for (letter_index, letter) in word.chars().enumerate(){
						
						if letter_index >= aux.len(){
							break;
						};
							match letter{
								letter if letter == aux.chars().nth(letter_index).unwrap() => answer.push(letter),
								_ => break,
							};
					};

					if word_index != strs.iter().len() - 1{
						aux = answer.clone();
						answer = "".to_string();

					}else{
						continue;
					};


				}else{
					if flag == true{
						break;
					}
					aux = word.clone();
					flag = true;
				};
			}
		}
		answer

}


fn main(){
	let strs: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть fl");
	println!("результат {}", result);
	println!("");

	let strs:Vec<String> = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть ()");
	println!("Ещё один тест {}", result);
	println!("");

	let strs:Vec<String> = vec!["a".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть a");
	println!("Ещё один тест {}", result);
	println!("");

	let strs:Vec<String> = vec!["aaa".to_string(),"aa".to_string(),"aaa".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть aa");
	println!("Ещё один тест {}", result);
	println!("");
	// Проблема
	let strs:Vec<String> = vec!["caa".to_string(),"".to_string(),"a".to_string(),"acb".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть ()");
	println!("Ещё один тест {}", result);
	println!("");

	
	let strs:Vec<String> = vec!["ca".to_string(),"c".to_string(),"bba".to_string(),"bacb".to_string(),"bcb".to_string()];
	let result = longest_common_prefix(strs);
	println!("Должен быть ()");
	println!("Ещё один тест {}", result);

}
