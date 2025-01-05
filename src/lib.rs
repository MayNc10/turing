use std::{error::Error, fmt::{Debug, Display}, fs::read_to_string, io::{stdin, stdout, Write}, path::PathBuf};

use petgraph::{csr::DefaultIx, graph::NodeIndex, Graph};
use state::State;
use tape::TapeTransition;

pub mod cli;
pub mod state;
pub mod tape;

pub fn loop_read<T, F, P>(
    buf: &mut String, msg: &str, repeat_msg: &str, transform: F, predicate: P) -> T
where 
    F: Fn(&String) -> T,
    P: Fn(&T) -> bool,
{
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(buf).unwrap();
    let mut val = transform(buf);
    buf.clear();
    while !predicate(&val) {
        print!("{}", repeat_msg);
        stdout().flush().unwrap();
        stdin().read_line(buf).unwrap();
        val = transform(buf);
        buf.clear();
    }
    val
}

pub fn loop_read_res<T, S: Display + Debug, F>(
    buf: &mut String, msg: &str, transform: F) -> T
where 
    F: Fn(&String) -> Result<T, S>
{
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(buf).unwrap();
    let mut val = transform(buf);
    buf.clear();
    while let Err(err) = val {
        print!("{}", err);
        stdout().flush().unwrap();
        stdin().read_line(buf).unwrap();
        val = transform(buf);
        buf.clear();
    }
    val.unwrap()
}

pub fn from_dot(path: &PathBuf) -> Result<Graph<State, TapeTransition>, Box<dyn Error>> {
    let dotfile = read_to_string(path)?;
    let lines = dotfile.split("\n").skip(1);
    let mut graph = Graph::new();
    for line in lines {
        let mut tokens = line.trim()
            .split(" ")
            .skip_while(|s| s.is_empty())
            .peekable();

        let first = tokens.next();
        if let Some(Ok(first_idx)) = first.map(|s| s.parse::<DefaultIx>()) {
            let first_idx = NodeIndex::from(first_idx);

            match tokens.peek() {
                Some(&"[") => {
                    // parse state
                    graph.add_node(
                        State::<DefaultIx>::from_node(tokens).ok_or("State parsing failed")?);
                },
                Some(&"->") => {
                    let mut tokens = tokens.skip(1);
                    let second_idx = NodeIndex::from(tokens.next()
                        .ok_or("Missing edge end")?
                        .parse::<DefaultIx>()?);
                    let (trans, map) = 
                        TapeTransition::from_edge(tokens)?;
                    // modify state to make maps
                    graph[first_idx].as_transition_mut().unwrap()
                        .add_transition(map.iter(), second_idx);
                    graph.add_edge(first_idx, second_idx, trans);
                },
                _ => (),
            }
        }
    }

    Ok(graph)
}