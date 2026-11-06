# Upgrades

## 0.2.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, translator 0.3.0

Repins signal 10.3.0 (7afb), datom-codec 0.33.0 (eabe), protos 0.33.0 (3c7a) and translator 0.3.0 (eeaa), each spelled as translator spells them so the lock holds one source of each. The translator moves from 0.2.0 (7078); the contract uses only `translator::PeerBinding` and `translator::PeerId`, whose shapes are unchanged.

- The generated contract is regenerated with ethos-zero 19.2.0 (5a5d), the flake's generator input now the bare URL with the lock pinning it. The authored `signal.ethos` is unchanged; the generated types add `PartialOrd` and `Ord` (and `Copy` where the scalar is `Copy`). No type, variant or field changes, and the rkyv layout is unchanged.
- protos 0.33.0 prints a balanced guillemet pair inside a string without an escape; a consumer holding datom text with an unbalanced guillemet re-reads it.
- The flake keeps its `generated` and `test` checks and gains `test-default` and `test-datom`. `tests/contract.rs` archives and restores a query and a response in every build, and under `datom` reads, prints and refuses wrong shapes for the configure and response forms.

To deploy: repin meta-signal-herdr-nexus 0.2.0 in the consumer, with the same translator revision.
