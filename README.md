# holonomy-harmony-rs

A pure Rust port of [holonomy-harmony](https://github.com/SuperInstance/holonomy-harmony) — holonomy groups, parallel transport, fiber bundles, and algebraic topology applied to musical traditions.

## Features

- **Tonal graph** — Directed weighted graph over pitch classes with transition classification
- **Simplicial complexes** — For representing harmonic spaces
- **Betti numbers** — Topological invariants (connected components, loops)
- **Curvature flow** — Forman-Ricci-like curvature on tonal graphs
- **Holonomy computation** — Net winding around the circle of fifths
- **Progression classification** — Diatonic, chromatic mediant, chromatic

## Usage

```rust
use holonomy_harmony::{TonalGraph, SimplicialComplex, BettiNumbers, CurvatureFlow};

let mut g = TonalGraph::new();
g.build_from_progression(&[0, 5, 7, 0]); // I-IV-V-I

let flow = CurvatureFlow::compute(&[0, 5, 7, 0]);
println!("Average curvature: {}", flow.avg_curvature());
```

## License
MIT
