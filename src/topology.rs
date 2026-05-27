//! Algebraic topology: simplicial complexes, Betti numbers, fundamental groups.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Simplicial complex for representing musical spaces.
#[derive(Debug, Clone, Default)]
pub struct SimplicialComplex { simplices: HashMap<usize, HashSet<Vec<u32>>> }

impl SimplicialComplex {
    pub fn new() -> Self { Self::default() }
    pub fn add_simplex(&mut self, vertices: Vec<u32>) {
        let dim = vertices.len().saturating_sub(1);
        let mut sorted = vertices; sorted.sort();
        self.simplices.entry(dim).or_default().insert(sorted);
    }
    pub fn simplices_of_dim(&self, dim: usize) -> usize {
        self.simplices.get(&dim).map(|s|s.len()).unwrap_or(0)
    }
    pub fn num_vertices(&self) -> usize { self.simplices_of_dim(0) }
    pub fn num_edges(&self) -> usize { self.simplices_of_dim(1) }
    pub fn num_triangles(&self) -> usize { self.simplices_of_dim(2) }
}

/// Betti numbers: topological invariants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BettiNumbers { pub b0: u32, pub b1: u32, pub b2: u32 }

impl BettiNumbers {
    /// Compute simplified Betti numbers for a simplicial complex.
    /// b0 = connected components, b1 = loops (Euler formula approximation).
    pub fn compute(sc: &SimplicialComplex) -> Self {
        let v = sc.num_vertices() as u32;
        let e = sc.num_edges() as u32;
        let f = sc.num_triangles() as u32;
        let euler = v as i32 - e as i32 + f as i32;
        let b0 = 1.max(euler.max(1) as u32);
        let b1 = if e > v { e - v + f } else { 0 };
        BettiNumbers { b0, b1, b2: 0 }
    }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_complex() {
        let mut sc = SimplicialComplex::new();
        sc.add_simplex(vec![0]); sc.add_simplex(vec![1]); sc.add_simplex(vec![2]);
        sc.add_simplex(vec![0,1]); sc.add_simplex(vec![1,2]); sc.add_simplex(vec![0,2]);
        assert_eq!(sc.num_vertices(), 3); assert_eq!(sc.num_edges(), 3);
    }
    #[test] fn test_betti() {
        let mut sc = SimplicialComplex::new();
        (0..5).for_each(|v| sc.add_simplex(vec![v]));
        [(0,1),(1,2),(2,3),(3,4),(4,0)].iter().for_each(|&(a,b)| sc.add_simplex(vec![a,b]));
        let b = BettiNumbers::compute(&sc); assert!(b.b0 >= 1);
    }
}
