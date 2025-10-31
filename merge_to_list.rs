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


// The reference solution 

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // Dummy node
        // tail = dummy
        // while l1 and l2: 
            // if l1.val < l2.val:
                // Calling Recursivley
                // append l1
            // else 
                // Calling Recursivley
                // append l2
        // appened remaining nodes
        // return dummy.next
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    let next = Self::merge_two_lists(n1.next.take(), Some(n2));
                    n1.next = next;
                    Some(n1)
                } else {
                    let next = Self::merge_two_lists(Some(n1), n2.next.take());
                    n2.next = next;
                    Some(n2)
                }
            }
        }
    } 
}