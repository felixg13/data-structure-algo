mod graph;
use graph::create_example_graph;

fn print_array(title: &str, array: &[&str]) {
    print!("{}: [", title);
    for (i, vid) in array.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{}", vid);
    }
    println!("]");
}

fn main() {
    let g = create_example_graph();

    print_array("Breadth First Search", &g.bfs("A"));
    print_array("Depth First Search", &g.dfs("A"));
}


