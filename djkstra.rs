use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

// Custom struct for priority queue(we want min heap so reverse ordering)
#[derive(Eq, PartialEq)]
struct Node {
    id: usize,
    distance: i32,
}
// Implement ordering for min heap behaviour
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse for min heap
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
