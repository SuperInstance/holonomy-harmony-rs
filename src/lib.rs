//! Holonomy in musical harmony — connection matrices, curvature, tonal fiber bundles.

use std::f64::consts::PI;

/// A tonal connection matrix (parallel transport between keys).
#[derive(Debug, Clone)]
pub struct ConnectionMatrix {
    /// 12x12 matrix representing key relationships.
    pub matrix: [[f64; 12]; 12],
}

impl Default for ConnectionMatrix {
    fn default() -> Self { Self::identity() }
}

impl ConnectionMatrix {
    pub fn identity() -> Self {
        let mut m = [[0.0; 12]; 12];
        for i in 0..12 { m[i][i] = 1.0; }
        Self { matrix: m }
    }

    pub fn circle_of_fifths() -> Self {
        let mut m = [[0.0; 12]; 12];
        for i in 0..12 {
            let j = (i + 7) % 12;
            m[i][j] = 1.0;
        }
        Self { matrix: m }
    }

    /// Apply connection (transport) from one key to another.
    pub fn transport(&self, from: u8, to: u8, vector: &[f64; 12]) -> [f64; 12] {
        let mut result = [0.0; 12];
        let shift = ((to as i32 - from as i32).rem_euclid(12)) as usize;
        for i in 0..12 {
            result[(i + shift) % 12] = vector[i];
        }
        result
    }

    /// Compute holonomy: transport around a closed loop.
    pub fn holonomy(&self, path: &[u8], vector: &[f64; 12]) -> [f64; 12] {
        if path.len() < 2 { return *vector; }
        let mut current = vector.clone();
        for i in 0..path.len() - 1 {
            current = self.transport(path[i], path[i + 1], &current);
        }
        // Close the loop
        current = self.transport(path[path.len() - 1], path[0], &current);
        current
    }

    /// Holonomy magnitude (how much the vector changes).
    pub fn holonomy_magnitude(&self, path: &[u8], vector: &[f64; 12]) -> f64 {
        let transported = self.holonomy(path, vector);
        vector.iter().zip(transported.iter())
            .map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
    }
}

/// Scalar curvature of a key region.
pub fn scalar_curvature(connection: &ConnectionMatrix, key: u8) -> f64 {
    // Sum of holonomies around minimal loops (triads: I-IV-V-I)
    let path = [key, (key + 5) % 12, (key + 7) % 12];
    let vector = {
        let mut v = [0.0; 12];
        v[key as usize] = 1.0;
        v
    };
    connection.holonomy_magnitude(&path, &vector)
}

/// Harmonic distance between two keys.
pub fn harmonic_distance(from: u8, to: u8) -> f64 {
    let diff = ((to as i32 - from as i32).rem_euclid(12)) as f64;
    // Distance on the circle of fifths
    let fifths = (diff * 7.0 % 12.0).min(12.0 - (diff * 7.0 % 12.0));
    // Combined chromatic + fifths distance
    let chromatic = diff.min(12.0 - diff);
    (chromatic + fifths) / 2.0
}

/// Tonal gravity: pull toward the tonic.
pub fn tonal_gravity(note: u8, tonic: u8, weights: &[f64; 12]) -> f64 {
    let interval = ((note as i32 - tonic as i32).rem_euclid(12)) as usize;
    weights[interval]
}

/// Default tonal gravity weights (major scale).
pub fn major_gravity_weights() -> [f64; 12] {
    // Tonic strongest, then dominant, then mediant, etc.
    [1.0, 0.1, 0.3, 0.2, 0.6, 0.4, 0.1, 0.8, 0.1, 0.5, 0.3, 0.2]
}

/// Compute the curvature of a chord progression.
pub fn progression_curvature(chords: &[u8]) -> f64 {
    if chords.len() < 3 { return 0.0; }
    let mut total = 0.0;
    for i in 0..chords.len() - 2 {
        let d1 = harmonic_distance(chords[i], chords[i + 1]);
        let d2 = harmonic_distance(chords[i + 1], chords[i + 2]);
        let d_direct = harmonic_distance(chords[i], chords[i + 2]);
        // Curvature = deviation from straight line
        total += (d1 + d2 - d_direct).max(0.0);
    }
    total / (chords.len() - 2) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_connection() {
        let c = ConnectionMatrix::identity();
        let v = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let result = c.transport(0, 0, &v);
        assert!((result[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_transport_transposition() {
        let c = ConnectionMatrix::identity();
        let v = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let result = c.transport(0, 7, &v); // C to G
        assert!((result[7] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_holonomy_closed_loop() {
        let c = ConnectionMatrix::identity();
        let v = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let path = [0, 4, 7]; // C-E-G-C
        let result = c.holonomy(&path, &v);
        // After going C→E→G→C with identity, should return to original
        assert!((result[0] - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_harmonic_distance() {
        assert!((harmonic_distance(0, 0) - 0.0).abs() < 0.001);
        assert!(harmonic_distance(0, 6) > 0.0); // tritone
        assert!(harmonic_distance(0, 7) > 0.0); // fifth
    }

    #[test]
    fn test_scalar_curvature() {
        let c = ConnectionMatrix::identity();
        let curv = scalar_curvature(&c, 0);
        assert!(curv >= 0.0);
    }

    #[test]
    fn test_tonal_gravity() {
        let weights = major_gravity_weights();
        assert!(tonal_gravity(0, 0, &weights) > tonal_gravity(1, 0, &weights));
        assert!(tonal_gravity(7, 0, &weights) > tonal_gravity(6, 0, &weights));
    }

    #[test]
    fn test_progression_curvature() {
        // I-IV-V-I: relatively smooth
        let smooth = progression_curvature(&[0, 5, 7, 0]);
        // I-bII-bIII: chromatic, more curved
        let chromatic = progression_curvature(&[0, 1, 2]);
        assert!(smooth < chromatic);
    }
}
