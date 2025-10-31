#[derive(Debug)]
pub struct ListNode {
	pub val: i32,
	pub next: Option<Box<ListNode>>
}

impl ListNode{
	fn new(val: i32) -> Self{
		ListNode{val, next: None}
	}

	fn append(&mut self, val: i32){
		let mut current = self;

		while let Some(ref mut next) = current.next {
			current = next;
		}

		current.next = Some(Box::new(ListNode::new(val)));
	}
}

fn main(){
	
	pub fn merge_to_list(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
			
			pub fn iter_to_linked_list(list: Option<Box<ListNode>>, integer_vector:&mut Vec<i32>){
				let mut current = list;	
			
				while let Some(node) = current {
					let num = node.val;
					integer_vector.push(num);
					current = node.next;
				}
			}
			let mut integer_vector: Vec<i32> = vec![];
			iter_to_linked_list(list1, &mut integer_vector);
			iter_to_linked_list(list2, &mut integer_vector);
			
			if integer_vector.len() == 0{
				return None
			};
			
			integer_vector.sort();
			let first_num = integer_vector.remove(0);
			let mut list_node: ListNode = ListNode::new(first_num);


			for num in integer_vector.iter(){
				list_node.append(*num);
			}
		Some(Box::new(list_node))
	}
}