use std::{collections::HashMap, fmt::{Debug, Display}};

use petgraph::{csr::{DefaultIx, IndexType}, graph::NodeIndex};

pub struct Transition<Idx: IndexType = DefaultIx> {
    map: HashMap<char,  NodeIndex<Idx>>,
    self_idx: NodeIndex<Idx>,
}

impl<Idx: IndexType> Transition<Idx> {
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

    pub fn next_state<'a, T, F>(&self, input: &char, indexer: &'a T, idx_fun: F) -> Option<&'a State<Idx>>
    where F: Fn(&'a T,  NodeIndex<Idx>) -> &'a State<Idx>
    {
        self.map.get(input)
            .map(|idx| idx_fun(indexer, idx.clone()))
    }

    pub fn set_idx(&mut self, idx:  NodeIndex<Idx>) {
        self.self_idx = idx;
    }
}

pub enum State<Idx: IndexType = DefaultIx> {
    Transition(Transition<Idx>),
    Accept,
    Reject,
}

impl<Idx: IndexType> State<Idx> {
    pub fn as_transition(&mut self) -> Option<&mut Transition<Idx>> {
        match self {
            Self::Transition(t) => Some(t),
            _ => None,
        }
    }
}

impl<Idx: IndexType> Display for Transition<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "q{:?}", self.self_idx.index())
    }
}

impl<Idx: IndexType> Display for State<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Transition(t) => t.fmt(f),
            State::Accept => write!(f, "qa"),
            State::Reject => write!(f, "r"),
        }
    }
}

impl<Idx: IndexType> Debug for State<Idx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}