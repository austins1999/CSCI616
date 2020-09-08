
fn main() {

// Read input of comma-separated node names and save to new vector
    let mut input = String::new();
    for arg in std::env::args().skip(1) {
        input += &arg;
    }
    let nodes: Vec<&str> = input.split(",").collect();

// Initialize string to contain the Graphviz definition
    let mut graph_str = "
digraph {
\trankdir=LR;

\tnode [shape=point]; start;
\tnode [shape=doublecircle]; ".to_owned() + &nodes[nodes.len()-1] + ";
\tnode [shape=circle];

\tstart -> " + &nodes[0] + ";\n";

// Link together all nodes then print final string
    for i in 0..(nodes.len()-1) {
        graph_str += &("\t".to_owned() + &(nodes[i].to_owned() + " -> " + &nodes[i+1] + ";\n"));
        // (this line seems really inefficient, but it works)
    }

    graph_str += "}\n";
    println!("{}", graph_str)
}
