//! Curvature flow on tonal graphs.

use std::collections::HashMap;

/// Curvature measure on an edge.
#[derive(Debug, Clone, Copy)]
pub struct CurvatureMeasure { pub edge: (u8, u8), pub curvature: f64 }

/// Curvature flow for computing Forman-Ricci-like curvature on tonal graphs.
pub struct CurvatureFlow { measures: Vec<CurvatureMeasure> }

impl CurvatureFlow {
    pub fn new() -> Self { Self { measures: vec![] } }
    /// Compute curvature for each edge in a tonal graph progression.
    pub fn compute(roots: &[u8]) -> Self {
        let mut degrees: HashMap<u8, u32> = HashMap::new();
        for i in 0..roots.len() { *degrees.entry(roots[i]%12).or_insert(0) += 1; }
        let mut measures = Vec::new();
        for i in 1..roots.len() {
            let (f, t) = (roots[i-1]%12, roots[i]%12);
            let df = *degrees.get(&f).unwrap_or(&1) as f64;
            let dt = *degrees.get(&t).unwrap_or(&1) as f64;
            let curv = 4.0 - df - dt;
            measures.push(CurvatureMeasure { edge: (f, t), curvature: curv });
        }
        Self { measures }
    }
    pub fn total_curvature(&self) -> f64 { self.measures.iter().map(|m| m.curvature).sum() }
    pub fn avg_curvature(&self) -> f64 {
        if self.measures.is_empty() { 0.0 } else { self.total_curvature() / self.measures.len() as f64 }
    }
    pub fn measures(&self) -> &[CurvatureMeasure] { &self.measures }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_flow() { let f = CurvatureFlow::compute(&[0,5,7,0]); assert!(f.total_curvature() != 0.0); }
    #[test] fn test_empty() { let f = CurvatureFlow::compute(&[0]); assert_eq!(f.measures().len(), 0); }
}
