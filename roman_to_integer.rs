use std::collections::HashMap;


pub fn main(s:String) -> Result<i32>{
	let mut  romanian_integers: HashMap<String, i32> = HashMap::new();
	let mut answer: i32 = 0;

	let mut aux: i32 = 0;
	let mut inner_aux: i32 = 0;	

	romanian_integers.insert("I", 1);
	romanian_integers.insert("V", 5);
	romanian_integers.insert("X", 10);
	romanian_integers.insert("L", 50
	);
	romanian_integers.insert("C", 100);
	romanian_integers.insert("D", 500);
	romanian_integers.insert("M", 1000);

	for (rom_index, rom_num) in s.chars().enumerate(){

		if s.len() == rom_index{
			answer += romanian_integers.get(rom_num);
		}else{
			aux = romanian_integers.get(rom_num);
			inner_aux = romanian_integers.get(s.chars()[rom_index + 1]);
			if aux > inner_aux{
				answer += aux;
			} else {
				answer += inner_aux - aux;
			};
		};
		
	}
	answer
	
}
