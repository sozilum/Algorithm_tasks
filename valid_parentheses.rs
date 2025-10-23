
pub fn valid_parenthese(s: String) -> bool{
	let mut asnwer: bool = false;
	let mut aux_string: String = String::from::new();
	let mut aux_num: i32 = 0;
	let mut some_aux: String = String::from::new();

	match s{
		s if s.len() % 2 != 0 => answer = false;
		s if s.len() % 2 == 0 => {
			aux_num = s.len() / 2;

			aux_string = s[aux_num..];

			for symbol in aux_string{

				match symbol{
					symbol if symbol == ")" => some_aux.push("(");
					symbol if symbol == "]" => some_aux.push("[");
					symbol if symbol == "}" => some_aux.push("{");
				}
			}

			if some_aux == aux[..aux_num]{
				answer = true;
			}
		}
		_ => panic!("something gone wrong")
	}
	answer
	
}



fn main(){

	let s = "()";
	println!("must be true");
	let answer = valid_parentheses(s);
	println!("asnwer {}", asnwer.to_string());

	let s = "()[]{}";
	println!("must be true");
	let answer = valid_parentheses(s);
	println!("asnwer {}", asnwer.to_string());

	let s = "(]";
	println!("must be false");
	let answer = valid_parentheses(s);
	println!("asnwer {}", asnwer.to_string());

	let s = "([])";
	println!("must be true");
	let answer = valid_parentheses(s);
	println!("asnwer {}", asnwer.to_string());

	let s = "([)])";
	println!("must be true");
	let answer = valid_parentheses(s);
	println!("asnwer {}", asnwer.to_string());
	
}
