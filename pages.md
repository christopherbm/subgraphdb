# subgraphdb page structure

### Notes
  - Assuming page size of 4096 bytes but this should be configurable. Page size must be divisible by 8.
  - All row types allowed in a page must all be the same length.
  - Most of this logic is under the /datagram workspace.
  - Every Page has a Page Type.
  - Spaces below are just for readability.
  - Page size is configurable; row size is fixed.
  - Graph indexes will be a separate page
  - Node and Edge properties are also a seperate page.
  - Row lengths must be an even factor of 8.


### Database Config Page
  - SDB build id and database nickname
  - Graph Reference: graph nickname
```
[PGESTR] [::SDBC] build_id nickname [::::WC]
[::::GR] nickname [::::WC]
...
[PGEEND] [::SDBC] nickname [::::WC]
```

### Graph Nodes Page
```
[PGESTR] [::GNPG] [::::WC]
[PGEEND] [::GNPG] [::::WC]
```

### Graph Edges Page
```
[PGESTR] [::GEPG] [::::WC]
[PGEEND] [::GEPG] [::::WC]
```

### N/E Properties Page