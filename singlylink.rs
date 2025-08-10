struct LinkNode {
    value: i32,
    next: Option<Box<LinkNode>>
}

impl LinkNode {
    fn new(value: i32) -> Self {
        LinkNode {
            value, 
            next: None
        }
    }
}

fn reverse_list(head: Option<Box<LinkNode>>) -> Option<Box<LinkNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node)= current {
        let next = node.next.take(); 
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    prev
}
fn create_list(vec: Vec<i32>) -> Option<Box<LinkNode>> {
    let mut head = None;
    let mut current = &mut head;

    for &value in vec.iter().rev() {
        let new_node = Box::new(LinkNode{
            value: value, 
            next: current.take()
        });
        *current = Some(new_node);
        current = &mut current.as_mut().unwrap().next;
    }
    head
}
fn print_list(head: &Option<Box<LinkNode>>) {
    let mut current = head;
    while let Some(node) = current {
        println!("{} ", node.value);
        current = &node.next;
    }
    println!{"None"};

}
fn main() {
    let vec = vec![1,2,3,4,5,6,7];
    let head = create_list(vec);
    println!("Original list:");
    print_list(&head);
    let reversed_list = reverse_list(head);
    println!("Reversed list:");
    print_list(&reversed_list);
}