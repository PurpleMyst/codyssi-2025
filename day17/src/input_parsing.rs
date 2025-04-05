use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use petgraph::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct StaircaseName(pub(crate) u8);

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct StaircaseStep(pub(crate) u8, pub(crate) u8);

impl FromStr for StaircaseName {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.strip_prefix("S").unwrap_or(s).parse()?))
    }
}

impl Display for StaircaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}", self.0)
    }
}

impl Debug for StaircaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl FromStr for StaircaseStep {
    type Err = <u8 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split('_').map(|x| x.strip_prefix("S").unwrap_or(x).parse());
        Ok(Self(it.next().unwrap()?, it.next().unwrap()?))
    }
}

impl Display for StaircaseStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}_{}", self.0, self.1)
    }
}

impl Debug for StaircaseStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

pub(crate) type StepGraph = GraphMap<StaircaseStep, u8, Directed, rustc_hash::FxBuildHasher>;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Staircase {
    pub(crate) name: StaircaseName,
    pub(crate) start: u8,
    pub(crate) end: u8,
    pub(crate) feeding: StaircaseName,
    pub(crate) returning: StaircaseName,
}

#[derive(Debug, Clone)]
pub(crate) struct ProblemStatement {
    pub(crate) s1_end: u8,
    pub(crate) staircases: Vec<Staircase>,
    pub(crate) allowed_moves: Vec<u8>,
}

/// Load a problem statement, returning the N in `S1_N` and all the branching staircases.
pub(crate) fn load_input(input: &str) -> ProblemStatement {
    let mut ls = input.lines();
    let allowed_moves = ls
        .next_back()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    debug_assert!(allowed_moves.contains(&1));
    let l = ls.next().unwrap();
    let mut it = l.split_ascii_whitespace().skip(2).step_by(2);
    let s1_start = it.next().unwrap().parse::<u8>().unwrap();
    debug_assert_eq!(s1_start, 0);
    let s1_end = it.next().unwrap().parse().unwrap();

    let mut staircases = vec![];

    for l in ls.take_while(|l| !l.is_empty()) {
        let mut it = l.split_ascii_whitespace();
        let name = it.next().unwrap();
        let start = it.nth(1).unwrap().parse().unwrap();
        let end = it.nth(1).unwrap().parse().unwrap();
        let feeding = it.nth(2).unwrap();
        let returning = it.nth(1).unwrap();
        staircases.push(Staircase {
            name: name.parse().unwrap(),
            start,
            end,
            feeding: feeding.parse().unwrap(),
            returning: returning.parse().unwrap(),
        });
    }

    ProblemStatement {
        s1_end,
        staircases,
        allowed_moves,
    }
}
