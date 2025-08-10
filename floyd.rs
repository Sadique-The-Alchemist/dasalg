#[derive(Debug)]
struct LinkNode{
    value: i32,
    next: Option<Box<LinkNode>>     
}

fn has_cycle(head: Option<Box<LinkNode>>) -> bool {
    
   let mut slow = head.as_ref();
    let mut fast = head.as_ref();
    while let Some(fast_node) = fast {
        if let Some(next_fast_node) = &fast_node.next {

            fast = next_fast_node.next.as_ref();

            slow = slow.and_then(|node| node.next.as_ref());
            if let(Some(s), Some(f)) = (slow, fast){
                if std::ptr::eq(s, f){
                    return true;
                }
            }
        }
        else {
            break;
        }
    }
    false
}

fn main() {
    let mut node1 = Box::new(LinkNode { value: 1, next: None });
    let mut node2 = Box::new(LinkNode { value: 2, next: None });
    let mut node3 = Box::new(LinkNode { value: 3, next: None });
    node2.next = Some(node3);
    node1.next = Some(node2);
    let node3 = node1.next.as_mut().unwrap().next.as_mut().unwrap();
    node3.next = Some(node1); // Creating a cycle
    

    print!("Has cycle: {}", has_cycle(Some(node1)));
}