
fn main(){
	use std::thread::current;
	use std::collections::HashMap;

	impl<ListNode>  Solution{
		pub fn merge_to_list(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
			#[derive(Debug)]
			pub struct ListNode {
				val: i32,
				next: Option<Box<ListNode>>
			}

			impl ListNode{
				fn new(val: i32) -> Self{
					ListNode {val, next: None}
				}

				fn append(&mut self, val: i32){
					let mut current = self;

					while let Some(ref mut next) = current.next {
						current = next;
					}

					current.next = Some(Box::new(ListNode::new(val)));
				}
			}

			pub fn iter_to_linked_list(list: Option<Box<ListNode>>,&mut hashmap:&mut  HashMap<i32, i32>){
				while let Some(list) = current {

				match list.get(&list){
					Some(&num) => list.get(&num) += 1,
					_ => list.insert(num),
				}
				
				}
				let current = &list.unwrap();	
			}
			let mut integer_hashmap: HashMap<i32, i32> = HashMap::new();
			let mut list_node: ListNode = ListNode::new(<i32>);
			_ = iter_to_linked_list(list1, &integer_hashmap);
			_ = iter_to_linked_list(list2, &integer_hashmap);


			for keys in integer_hashmap.keys()(){

				for num in integer_hashmap.get(keys){

					list_node.append(*keys);
					
				}
			}
		list_node	
		}
	}
}








