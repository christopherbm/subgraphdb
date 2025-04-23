# subgraphdb
SubGraphDB. A flat-file graph database built with Rust.

### In Alpha Development

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
