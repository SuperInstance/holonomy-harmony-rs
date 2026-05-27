//! Chord analysis and holonomy computation for progressions.

/// Compute holonomy (net winding) around the circle of fifths.
pub fn compute_holonomy(roots: &[u8]) -> i32 {
    if roots.len() < 2 { return 0; }
    const CIRCLE: [u8; 12] = [5,0,7,2,9,4,11,6,1,8,3,10];
    fn pos(pc: u8) -> i32 { CIRCLE.iter().position(|&c| c==pc%12).unwrap() as i32 }
    let mut cum = 0i32;
    for i in 1..roots.len() {
        let d = (pos(roots[i]) - pos(roots[i-1]) + 6).rem_euclid(12) - 6;
        cum += d;
    }
    cum
}

/// Classify a progression type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressionType { Diatonic, ChromaticMediant, Chromatic }

pub fn classify_progression(from: u8, to: u8) -> ProgressionType {
    match (to as i32 - from as i32).rem_euclid(12) {
        0|2|4|5|7|9|11 => ProgressionType::Diatonic,
        3|8 => ProgressionType::ChromaticMediant,
        _ => ProgressionType::Chromatic,
    }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_hol() { assert!(compute_holonomy(&[0,5,7,0]).abs() <= 2); }
    #[test] fn test_class() { assert_eq!(classify_progression(0,7), ProgressionType::Diatonic); }
}
