# SubgraphDB
SubGraphDB. A flat-file graph database built with Rust.
Goal: Create a performant graph database that can run anywhere from a browse tab to a micro-controller.

### Lifecycle
- Pre-Alpha development. We will move to Alpha when a basic set of Cypher compatibility is met and the CLI is working.

### Workspaces
- API: central hub for beginning interactions with SDB
- CLI: CLI for sdb
- CMD:
- Common: common enums and structs used across SDB
- Datagram: responsible for converting data structures to appropriate byte arrays
- Executor: uses transactions to change persisted byte data in SDB 
- Planner: plans transactions from input to be executed against SDB
- SDB:
- SDB_Config:

### Working Notes
  - One of the goals of this project is to allow sub-graphs to be pulled out of larger graphs into their own complete databases (with a single query). Potentially the reverse is possible as well.
  - There is a default database to stay in sync with Cypher syntax. However, multiple distinct graphs can also be used.
  - Currently limiting max nodes for an entire database as a u64.

### Short-term Roadmap
  - 100% Cypher compatibility
  - Ability to turn a query (sub-graph) into an entire new database.
  - Ensure TDE is reasonably possible

### Long-term Roadmap
  - WASM compatibility
  - Transparent data encryption (TDE)
  - hyper-indexing: turn a sub-graph and a set of queries into a new read-only database with highly-tuned indexes.