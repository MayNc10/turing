use std::{collections::HashMap, fmt::{Debug, Display}, str::FromStr};

use petgraph::{csr::{DefaultIx, IndexType}, graph::NodeIndex};

pub struct Transition<Idx: IndexType = DefaultIx> {
    map: HashMap<char,  NodeIndex<Idx>>,
    self_idx: NodeIndex<Idx>,
}

impl<Idx: IndexType + FromStr> Transition<Idx> {
    pub fn new(map: HashMap<char, NodeIndex<Idx>>) -> Transition<Idx> {
        Transition { map, self_idx: NodeIndex::end() }
    }

    pub fn empty() -> Transition<Idx> {
        Transition { map: HashMap::new(), self_idx: NodeIndex::end() }
    }

    pub fn add_transition<'a, I: Iterator<Item = &'a char>>(&mut self, inputs: I, idx: NodeIndex<Idx>) {
        for c in inputs {
            self.map.insert(*c, idx.clone());
        }
    }

    pub fn next_index(&self, input: &char, ) -> NodeIndex<Idx>
    {
        self.map[input].clone()
    }

    pub fn set_idx(&mut self, idx:  NodeIndex<Idx>) {
        self.self_idx = idx;
    }
}

pub enum State<Idx: IndexType + FromStr = DefaultIx> {
    Transition(Transition<Idx>),
    Accept,
    Reject,
}

impl<Idx: IndexType + FromStr> State<Idx> {
    // we're passed an array of space seperated terms
    pub fn from_node<'a, I: Iterator<Item = &'a str>>(s_it: I) -> Option<State<Idx>> {
        // format looks like [, label, =, "q$num", ]
        let data_str = s_it.skip(3).next()?;
        if &data_str[1..2] == "q" {
            if &data_str[2..data_str.len() - 1] == "a" {
                Some(State::Accept)
            }
            else if &data_str[2..data_str.len() - 1] == "r" {
                Some(Self::Reject)
            }
            else if let Ok(num) = data_str[2..data_str.len() - 1].parse::<Idx>() {
                let mut trans = Transition::empty();
                trans.set_idx(NodeIndex::from(num));
                Some(Self::Transition(trans))
            } 
            else { None }
        }
        else { None }
    }

    pub fn as_transition(&self) -> Option<&Transition<Idx>> {
        match self {
            Self::Transition(t) => Some(t),
            _ => None,
        }
    }

    pub fn as_transition_mut(&mut self) -> Option<&mut Transition<Idx>> {
        match self {
            Self::Transition(t) => Some(t),
            _ => None,
        }
    }


    pub fn accepted(&self) -> Option<bool> {
        match self {
            Self::Accept => Some(true),
            Self::Reject => Some(false),
            Self::Transition(_) => None,
        }
    }
}

impl<Idx: IndexType + FromStr> Display for Transition<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "q{:?}", self.self_idx.index())
    }
}

impl<Idx: IndexType + FromStr> Display for State<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Transition(t) => t.fmt(f),
            State::Accept => write!(f, "qa"),
            State::Reject => write!(f, "qr"),
        }
    }
}

impl<Idx: IndexType + FromStr> Debug for State<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}