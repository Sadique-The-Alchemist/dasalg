use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value,
            next: None,
        }))
    }
}

fn detect_cycle(head: &Rc<RefCell<Node>>) -> bool {
    let mut slow = Rc::clone(head);
    let mut fast = Rc::clone(head);
    while let Some(ref next_slow) = slow.borrow().next {
        slow = Rc::clone(next_slow);
        if let Some(ref next_fast) = fast.borrow().next {
            fast = Rc::clone(next_fast);
            if let Some(ref next_fast_next) = fast.borrow().next {
                fast = Rc::clone(next_fast_next);
            } else {
                return false; // No cycle if fast cannot move two steps
            }
        } else {
            return false; // No cycle if fast cannot move
        };
        if Rc::ptr_eq(&slow, &fast) {
            return true; // Cycle detected
        };
    };
    false // No cycle detected
}

fn main() {
    let node1 = Node::new(1);
    let node2 = Node::new(2);
    let node3 = Node::new(3);

    node1.borrow_mut().next = Some(Rc::clone(&node2));
    node2.borrow_mut().next = Some(Rc::clone(&node3));

    node3.borrow_mut().next = Some(Rc::clone(&node1)); // Creating a cycle

    let mut current = Some(Rc::clone(&node1));
    let mut count = 0;
    while let Some(node) = current {
        println!("Node value: {}", node.borrow().value);
        current = node.borrow().next.clone();
        count += 1;

        if count > 10 { // Prevent infinite loop in case of cycle
            println!("Cycle detected, stopping iteration.");
            break;
        }
    }


}