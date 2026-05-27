//! Tonal graph over pitch classes.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionDirection { Dominant, Subdominant, Chromatic }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonalGraph { edges: HashMap<(u8, u8), (f64, TransitionDirection)> }

impl TonalGraph {
    pub fn new() -> Self { Self::default() }
    pub fn add_transition(&mut self, from: u8, to: u8) {
        let d = classify(from, to);
        self.edges.entry((from%12,to%12)).or_insert((0.0,d)).0 += 1.0;
    }
    pub fn build_from_progression(&mut self, roots: &[u8]) {
        for i in 1..roots.len() { self.add_transition(roots[i-1], roots[i]); }
    }
    pub fn edge_count(&self) -> usize { self.edges.len() }
    pub fn weight(&self, from: u8, to: u8) -> Option<f64> { self.edges.get(&(from%12,to%12)).map(|(w,_)|*w) }
}

fn classify(from: u8, to: u8) -> TransitionDirection {
    match (to as i16 - from as i16).rem_euclid(12) { 7=>TransitionDirection::Dominant, 5=>TransitionDirection::Subdominant, _=>TransitionDirection::Chromatic }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_build() { let mut g=TonalGraph::new(); g.build_from_progression(&[0,5,7,0]); assert_eq!(g.edge_count(),3); }
}
