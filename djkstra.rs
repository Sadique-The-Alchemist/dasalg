use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

// Custom struct for priority queue(we want min heap so reverse ordering)
#[derive(Debug)]
struct Edge {
    id: usize,
    weight: i32,
}
// Implement ordering for min heap behaviour

#[derive(Eq, PartialEq)]
struct State{
    distance: i32,
    vertex: usize 
}

// Reverse ordering of heap smaller distances first

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Reverse for min heap
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Graph struct with adjacency list 
struct Graph{
    adj: Vec<Vec<Edge>>,
    vertices: usize 
}
impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            adj: vec![vec![]; vertices],
            vertices
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        self.adj[from].push(Edge {to, weight});
    }
    fn dijkstra(&self, start: usize) -> (Vec<i32>, Vec<Option<usize>>){
        let mut distances = vec![i32::MAX; self.vertices]; 
        let mut prev = vec![None; self.vertices];
        let mut pq = BinaryHeap::new(); 

        distances[start]=0;
        pq.push(State{distance: 0, vertex: start});
        while let Some(State {distance, vertex}) = pq.pop() {
            // if distance is outdated skip 
            if distance > distances[vertex] {
                continue;
            }
            // Explore neighbours 
            for edge in &self.adj[vertex] {
                let new_dist = distance + edge.weight;
                if new_dist < distances[edge.to] {
                    distances[edge.to] = new_dist;
                    prev[edge.to]= Some(vertex);
                    pq.push(State{
                        distance: new_dist,
                        vertex: edge.to 
                    });
                }
            }   
        }
        (distances, prev)
    }

}
// Reconstruct path from predecessors 
fn get_path(prev: &[Option<usize>], mut target: usize) -> Vec<usize> {
    let mut path = Vec::new();
    while target != 0 {
        path.push(target);
        target = prev[target].unwrap_or(0);
        }
        path.push(0);
        path.reverse();
        path
}

fn main {
    // Create a graph with 5 vertices 
    let mut graph = Graph::new(5);
  // Add edges (A=0, B=1, C=2, D=3, E=4)
  graph.add_edge(0, 1, 4);  // A → B
  graph.add_edge(0, 2, 2);  // A → C
  graph.add_edge(1, 3, 3);  // B → D
  graph.add_edge(2, 1, 2);  // C → B
  graph.add_edge(2, 3, 5);  // C → D
  graph.add_edge(3, 4, 2);  // D → E
  graph.add_edge(4, 1, 1);  // E → B

  // Run Dijkstra from A (0)
  let (distances, prev) = graph.dijkstra(0);

  // Print results
  let labels = ['A', 'B', 'C', 'D', 'E'];
  for i in 0..5 {
      let path = get_path(&prev, i);
      println!(
          "Distance from A to {}: {}, Path: {}",
          labels[i],
          distances[i],
          path.iter()
              .map(|&x| labels[x].to_string())
              .collect::<Vec<_>>()
              .join(" → ")
      );
  }

}