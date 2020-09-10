use std::io::Write;

fn main() {
    let graph = graph_builder();
    println!("{}", graph);
}

fn graph_builder() -> String {

    // Read comma-separated node names from command line
    let mut input = String::new();
    for arg in std::env::args().skip(1) {
        input += &arg;
    }

    // Ensure input was provided and save to vector
    if input.len() == 0 {
        writeln!(std::io::stderr(), "Usage: Provide comma-separated list of node names at runtime")
            .unwrap();
        std::process::exit(1);
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

    // Link together all nodes then return final string
    for i in 0..(nodes.len()-1) {
        graph_str += &("\t\t".to_owned() + &(nodes[i].to_owned() + " -> " + &nodes[i+1] + ";\n"));
        // (this line seems really inefficient, but it works)
    }
    graph_str += "\t}\n";
    return graph_str;
}


#[test]
fn test_graph_builder() {
    let test_str = "
        digraph {
        \trankdir=LR;

        \tnode [shape=point]; start;
        \tnode [shape=doublecircle]; q3;
        \tnode [shape=circle];

        \tstart -> q1;
        \tq1 -> q2;
        \tq2 -> q3;
    }\n";

    assert_eq!(graph_builder(), test_str);
}
