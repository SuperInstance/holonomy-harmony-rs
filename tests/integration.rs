//! Integration tests for holonomy-harmony-rs

use holonomy_harmony::*;

#[test]
fn test_connection_matrix_default_is_identity() {
    let c = ConnectionMatrix::default();
    for i in 0..12 {
        assert!((c.matrix[i][i] - 1.0).abs() < 1e-10, "diagonal should be 1");
        for j in 0..12 {
            if i != j {
                assert!(c.matrix[i][j].abs() < 1e-10, "off-diagonal should be 0");
            }
        }
    }
}

#[test]
fn test_circle_of_fifths_transport() {
    let c = ConnectionMatrix::circle_of_fifths();
    let v = [1.0; 12];
    // Transport C(0) to G(7): shift by 7 semitones
    let result = c.transport(0, 7, &v);
    // Every element should still be 1.0 since input is uniform
    for i in 0..12 {
        assert!((result[i] - 1.0).abs() < 1e-10);
    }
}

#[test]
fn test_holonomy_trivial_loop() {
    let c = ConnectionMatrix::identity();
    let v = [0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // Going C→C (trivial loop)
    let result = c.holonomy(&[0], &v);
    assert!((result[1] - 1.0).abs() < 1e-10);
}

#[test]
fn test_harmonic_distance_symmetry() {
    for i in 0u8..12 {
        for j in 0u8..12 {
            let d1 = harmonic_distance(i, j);
            let d2 = harmonic_distance(j, i);
            assert!((d1 - d2).abs() < 1e-10, "harmonic distance should be symmetric: {i}->{j}");
        }
    }
}

#[test]
fn test_harmonic_distance_triangle_inequality() {
    for i in 0u8..12 {
        for j in 0u8..12 {
            for k in 0u8..12 {
                let dij = harmonic_distance(i, j);
                let djk = harmonic_distance(j, k);
                let dik = harmonic_distance(i, k);
                assert!(dik <= dij + djk + 0.01, "triangle inequality failed: {i},{j},{k}");
            }
        }
    }
}

#[test]
fn test_progression_curvature_monotonic() {
    // More chromatic progression should have higher curvature
    let diatonic = progression_curvature(&[0, 2, 4]);
    let chromatic = progression_curvature(&[0, 1, 2]);
    assert!(chromatic >= diatonic, "chromatic should be at least as curved");
}

#[test]
fn test_tonal_gravity_major_scale() {
    let w = major_gravity_weights();
    // Tonic should be highest
    assert!((w[0] - 1.0).abs() < 1e-10);
    // Dominant should be second highest
    assert!(w[7] > w[6]);
}

#[test]
fn test_scalar_curvature_nonneg() {
    let c = ConnectionMatrix::identity();
    for key in 0u8..12 {
        let curv = scalar_curvature(&c, key);
        assert!(curv >= 0.0, "curvature should be non-negative at key {key}");
    }
}
