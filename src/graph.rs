use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub struct Graph<Vid, E = (), V = ()> {
    vertices: HashMap<Vid, V>,
    adjacency: HashMap<Vid, Vec<(Vid, E)>>
}

impl<Vid, E, V> Graph<Vid, E, V>
where 
    Vid: Eq + Hash + Copy,
    V: Hash,
    E: Clone,
{
    pub fn new() -> Graph<Vid, E, V> {
        Graph { vertices: HashMap::new(), adjacency: HashMap::new() }
    }

    // pub fn push_vertex(&mut self, vid: Vid, vertex: V) {
    //     self.vertices.insert(vid, vertex);
    // }

    pub fn push_edge(&mut self, from: Vid, to: Vid, edge: E) {
        let adjacent_to_from = self.adjacency.entry(from).or_default();
        adjacent_to_from.push((to, edge));
    }

    pub fn push_undirected_edge(&mut self, from: Vid, to: Vid, edge: E) {
        self.push_edge(from, to, edge.clone());
        self.push_edge(to, from, edge);
    }

    pub fn bfs(&self, start: Vid) -> Vec<Vid> {
        let mut q: VecDeque<Vid> = VecDeque::new();
        let mut visited_set: HashSet<Vid> = HashSet::new();
        let mut visited: Vec<Vid> = Vec::new();

        q.push_back(start);
        visited_set.insert(start);

        while let Some(current) = q.pop_front() {
            visited.push(current);

            if let Some(neighbours) = self.adjacency.get(&current) {
                for (vid, _) in neighbours {
                    if visited_set.insert(*vid) {
                        q.push_back(*vid);
                    }
                }
            }
        }

        visited
    }

    pub fn dfs(&self, start: Vid) -> Vec<Vid> {
        let mut stack: Vec<Vid> = Vec::new();
        let mut visited_set: HashSet<Vid> = HashSet::new();
        let mut visited: Vec<Vid> = Vec::new();

        stack.push(start);

        while let Some(current) = stack.pop() {
            if visited_set.insert(current) {
                visited.push(current);

                if let Some(neighbours) = self.adjacency.get(&current) {
                    for (vid, _) in neighbours {
                        if !visited_set.contains(vid) {
                            stack.push(*vid);
                        }
                    }
                }
            }
        }

        visited
    }
}

impl<Vid, E> Graph<Vid, E, ()>
where
    Vid: Eq + Hash,
{
    pub fn push_vid(&mut self, vid: Vid) {
        self.vertices.insert(vid, ());
    }
}

pub fn create_example_graph() -> Graph<&'static str, String> {
    let mut g: Graph<&str, String> = Graph::new();
    g.push_vid("A");
    g.push_vid("B");
    g.push_vid("C");
    g.push_vid("D");
    g.push_vid("E");
    g.push_undirected_edge("A", "B", "A - B".to_string());
    g.push_undirected_edge("A", "D", "A - D".to_string());
    g.push_undirected_edge("B", "C", "B - C".to_string());
    g.push_undirected_edge("D", "E", "D - E".to_string());
    g
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_bfs() {
        let g = create_example_graph();

        let bfs_result = g.bfs("A");
        assert_eq!(bfs_result, vec!["A", "B", "D", "C", "E"]);
    }

    #[test]
    fn test_graph_dfs() {
        let g = create_example_graph();

        let dfs_result = g.dfs("A");
        assert_eq!(dfs_result, vec!["A", "D", "E", "B", "C"]);
    }
}
