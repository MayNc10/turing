use std::{collections::HashSet, fs::write, io::{stdin, stdout, Write}};

use clap::Parser;
use petgraph::{dot::Dot, graph::NodeIndex, Graph};
use turing::{cli::Cli, loop_read, loop_read_res, state::{State, Transition}, tape::{Direction, TapeTransition}};

fn num_states(buf: &mut String) -> u32 {
    print!("Please enter the number of states: ");
    stdout().flush().unwrap();

    stdin().read_line(buf).unwrap();

    let num = buf[0..buf.len() - 1].parse().expect("Expected a valid number");
    buf.clear();
    num
}

fn alphabet(buf: &mut String, ty: &str) -> HashSet<char> {
    print!("Please enter the characters in the {ty} alphabet, without any seperators:\n");
    stdout().flush().unwrap();
    stdin().read_line(buf).unwrap();
    let list = buf[0..buf.len() - 1].chars().collect::<HashSet<_>>();
    buf.clear();
    list
}

fn fill_state(buf: &mut String, idx: NodeIndex, alphabet: &HashSet<char>, num_states: u32, 
    graph: &mut Graph<State, TapeTransition>) 
{
    println!("q{}: ", idx.index() + 1);
    let mut left = alphabet.clone();
    while !left.is_empty() {
        if left != *alphabet {
            print!("Stop? (remaining inputs: ");
            let mut it = left.iter().peekable();
            while let Some(c) = it.next() {
                let s = {
                    if *c == ' ' { " (blank)".to_string() }
                    else { c.to_string() }
                };
                if it.peek().is_some() {
                    print!("{s}, ");
                }
                else {
                    print!("{s}): ");
                }
            }
            stdout().flush().unwrap();
            stdin().read_line(buf).unwrap();
            if ["y", "yes", "stop"].contains(&buf.trim().to_lowercase().as_str()) {
                // all other states are reject
                // reject is always at num_states + 1
                graph[idx].as_transition().unwrap()
                    .add_transition(left.iter(), (num_states + 1).into());
                buf.clear();
                return;
            }
            else if !["n", "no", "continue", "c", ""].contains(&buf.trim().to_lowercase().as_str()) {
                println!("Didn't parse {} as a y/n answer or space, continuing", buf.trim().to_lowercase().as_str());
            }
            buf.clear();
        }

        let inputs = loop_read(buf, 
            "\tinputs: ", 
            "\tinputs were invalid, please reenter: ", 
            |buf| buf[0..buf.len() - 1].chars().collect::<HashSet<_>>(), 
            |inputs| alphabet.is_superset(inputs)
        );

        let next = loop_read_res(buf, 
                "\tnext state: ", 
                |buf| {
                    let next_str = if buf.chars().next() == Some('q') {
                        &buf.trim()[1..]
                    } else { &buf.trim() };
                    if let Ok(next_idx) = next_str.parse::<u32>() {
                        if next_idx > num_states {
                            Err(format!("State given was out of bounds! State: {}", next_idx))
                        }
                        else {
                            Ok(NodeIndex::from(next_idx - 1))
                        }
                    }
                    else {
                        if next_str.to_lowercase() == "a" || next_str.to_lowercase() == "accept" {
                            Ok(NodeIndex::from(num_states))
                        } 
                        else if next_str.to_lowercase() == "r" || next_str.to_lowercase() == "reject" {
                            Ok(NodeIndex::from(num_states + 1))
                        } 
                        else {
                            Err(format!("Invalid next state: {}, parsed to: {}", buf.trim(), next_str))
                        }
                    }
                   
                }
        );

        let write = loop_read(buf, 
            "\tchar to write: ", 
            "\tchar is not in alphabet, please reenter: ", 
            |buf| buf[0..buf.len() - 1].chars().next(), 
            |c| c.is_none() || alphabet.contains(&c.unwrap())
        );

        let direction = loop_read_res(buf, 
            "\tdirection: ", 
            |buf| {
                if let Some(dir_char) = buf[0..buf.len() - 1].chars().next() {
                    let dir_char = dir_char.to_ascii_uppercase();
                    if dir_char == 'L' {
                        Ok(Direction::Left)
                    }
                    else if dir_char == 'R' {
                        Ok(Direction::Right)
                    }
                    else {
                        Err("\tdirection given was invalid, please reenter: ")
                    }
                }
                else {
                    Err("\tno direction was entered, please reenter: ")
                } 
            }
        );

        // add state map
        graph[idx].as_transition().unwrap()
            .add_transition(inputs.iter(), next);
        left = left.into_iter()
            .filter(|c| !inputs.contains(c))
            .map(|c| c).collect();
        // edge
        let disp_string = {
            let mut s = String::new();
            for (idx, char) in inputs.iter().enumerate() {
                s.push(*char);
                if idx < inputs.len() - 1 {
                    s.push(',');
                }
            }
            s.push_str(" \u{2192} ");
            if let Some(write_c) = write {
                s.push(write_c);
                s.push(',');
            }
            if direction == Direction::Left {
                s.push('L');
            }
            else {
                s.push('R');
            }
            s
        };
        let tape_edge = TapeTransition { write, direction, disp_string };
        graph.add_edge(idx, next, tape_edge);
        buf.clear();
    }
}

fn main() {
    let cli = Cli::parse();

    let graph =  if cli.input {
        let mut buf = String::new();
        let num_states = num_states(&mut buf);
        let input = alphabet(&mut buf, "input");
        let tape = alphabet(&mut buf, "tape");
        // ensure some correctness?
        if !tape.is_superset(&input) {
            println!("Input alphabet is not subset of tape alphabet");
            return;
        }
    
        // create graph
        let mut graph: Graph<State, TapeTransition> = Graph::new();
        for _ in 0..num_states {
            let idx = graph.add_node(State::Transition(Transition::empty()));
            graph[idx].as_transition().unwrap().set_idx(idx);
        }
        // add accept and reject
        graph.add_node(State::Accept);
        graph.add_node(State::Reject);
    
        // insert edges
        for idx in 0..num_states {
            fill_state(&mut buf, idx.into(), &tape, num_states, &mut graph);
        }

        graph
    } else {
        todo!("Reading from file not yet implemented");
    };

    if let Some(path) = cli.output {
        // output dot
        let dot = Dot::new(&graph);
        println!("Writing dot: ");
        println!("{}", dot);
        
        write(path, dot.to_string()).expect("Writing to file failed");
    }
}
