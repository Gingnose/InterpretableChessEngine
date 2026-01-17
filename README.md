# InterpretableChessEngine

[![CI](https://github.com/Gingnose/InterpretableChessEngine/actions/workflows/ci.yml/badge.svg)](https://github.com/Gingnose/InterpretableChessEngine/actions/workflows/ci.yml)

An interpretable chess engine using graph theory to explain its reasoning in human-understandable terms.

## Core Question

**いかにシンプルな評価関数で強くすることができるか？**

Simple evaluation functions enable:
- Human interpretability
- Easy generalization to variants
- Clear debugging and improvement paths

## Project Goals

1. **Interpretability**: Explain *why* moves are good in human terms
2. **Generalization**: Adapt to new piece types without manual tuning
3. **Graph-Theoretic Evaluation**: Use graph properties instead of black-box neural networks

## Key Features

- **Threat-Based Analysis**: Evaluate positions through threats (forks, pins, checks)
- **Graph Centrality**: Derive piece values dynamically from graph properties
- **Variant Support**: Works with standard chess + custom pieces (Amazon, Camel, etc.)
- **Explainable AI**: Every evaluation includes human-readable breakdown

## Development Status

- [x] **Phase 0**: Project foundation (in progress)
  - [x] Cargo setup
  - [x] Core types (Square, Delta, Color)
  - [x] CI/CD
  - [x] Benchmarking infrastructure
- [ ] **Phase 1**: Standard chess move generation
- [ ] **Phase 2**: Interpretable evaluation
- [ ] **Phase 3**: Variant support
- [ ] **Phase 4**: GUI

## Building

```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Run with optimizations
cargo build --release
```

## Architecture

See [PROJECT_SPEC.md](PROJECT_SPEC.md) for detailed technical specifications.

## License

MIT
