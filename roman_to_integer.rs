use std::collections::HashMap;

pub fn main(){

pub fn roman_to_iter(s:String) -> Option<i32>{
	let mut  romanian_integers: HashMap<char, i32> = HashMap::new();
	let mut answer: i32 = 0;
	let char_vect: Vec<char> = s.chars().collect();


	let mut aux: i32 = 0;
	let mut inner_aux: i32 = 0;	

	romanian_integers.insert('I', 1);
	romanian_integers.insert('V', 5);
	romanian_integers.insert('X', 10);
	romanian_integers.insert('L', 50);
	romanian_integers.insert('C', 100);
	romanian_integers.insert('D', 500);
	romanian_integers.insert('M', 1000);

	for (rom_index, rom_num) in char_vect.iter().enumerate(){

		if s.len() - 1 == rom_index{
			answer += romanian_integers.get(&rom_num).unwrap();
		}else{
			aux = *romanian_integers.get(&rom_num).unwrap();

			inner_aux = *romanian_integers.get(&char_vect[rom_index + 1]).unwrap();
			if aux >= inner_aux{
				answer += aux;
			} else {
				answer += inner_aux - aux;
				answer -= inner_aux;
			};
		};
		
	}
	Some(answer)
}
}
