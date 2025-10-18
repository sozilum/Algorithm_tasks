pub fn longest_common_prefix(strs: Vec<String>) -> String{
	let mut aux: String = String::new();
	let mut answer: String = String::new();

	for word in strs.iter(){

		if aux != "" {

			for (letter_index, letters) in word.chars().enumerate(){
				if letters == aux.chars().nth(letter_index).unwrap(){
					answer.push(letters);
				}else{
					break;
				};

			}

			aux = answer;

		}else{
			aux = word.clone();
		};

	}
	answer

}


fn main(){
	let strs: Vec<String> = vec!["flower".to_string(),"flow".to_string(),"flight".to_string()];
	let result = longest_common_prefix(strs);
	println!("результат {}", result);

	let another_strs:Vec<String> = vec!["dog".to_string(),"racecar".to_string(),"car".to_string()];
	let another_result = longest_common_prefix(another_strs);
	println!("Ещё один тест {}", another_result);
}
