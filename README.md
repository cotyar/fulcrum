# Fulcrum
Distributed ultra scalable Database 

# plans

* Typed Trees that support working directly with serde-friendly types instead of raw bytes,
  and also allow the deserialized form to be stored in the shared cache for speedy access.
* [LSM tree](https://en.wikipedia.org/wiki/Log-structured_merge-tree)-like write performance
  with [traditional B+ tree](https://en.wikipedia.org/wiki/B%2B_tree)-like read performance
* MVCC and snapshots
* forward-compatible binary format
* concurrent snapshot delta generation and recovery
* consensus protocol for [PC/EC](https://en.wikipedia.org/wiki/PACELC_theorem) systems
* pluggable conflict detection and resolution strategies for gossip + CRDT-based [PA/EL](https://en.wikipedia.org/wiki/PACELC_theorem) systems
* first-class programmatic access to replication stream