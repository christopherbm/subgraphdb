/*
Chapter 1 - Create, Configure and Interact with SubgraphDB. 
  Section 1 - In-Memory Database
    - / Initialize (Create) In-Memory Database (Default Configuration)
  Section 2 - Single-File Database
    - / Initialize (Create) Database (Default Configuration)
    - / Initialize (Load) Database
  Section 3 - Multi-File Database
    - / Initialize (Create) Database (Default Configuration)
    - / Initialize (Load) Database
  Section 4 - Interacting with SubgraphDB
    - / List Graphs (IM/SF/MF)
    - / Create New Graph (IM/SF/MF)
    - / Load Graph (SF/MF)
---
Chapter 2 - Create and Configure Graph Data  
  Section 1 - Nodes and Edges  
    - / Create Notes and Edges (IM/SF/MF)
  Section 2 - Constraints
  Section 3 - Indexing
    - / Create Show Delete Indexes (IM/SF/MF)
  Section 4 - Schema
---
Chapter 3 - Query Graph Data

---
Chapter 4 - Import / Export

---
Chapter 5 - CLI
  // create must happen or error
  sdb -- /path/to/file.sdb -sf

  // load must happen or error
  sdb -- /path/to/file.sdb

  // folder and database must exist or error
  sdb -- /path/to/folder 

  // folder must exist or error
  sdb -- /path/to/folder -mf

  // this will default to interactive-mode
  sdb -im

  sdb -- -help

---
Chapter A - Parsing Functionality

---
Chapter B - Binary Functionality

---
Chapter C - Utility Functions
*/

/*
  Notes
    Group 1
      - Pages do not have to be in order.
      - Will likely need the concept of a "lease" on writable resources or discrete sections like pages.
      - Pages can also be linked-lists.
*/

/*
  CREATE GRAPH devs

  MATCH (n:Person {name:'Anna'})
  RETURN n.born AS birthYear
  FROM people

  MATCH (:Person {name: 'Anna'})-[r:KNOWS WHERE r.since < 2020]->(friend:Person)
  RETURN count(r) As numberOfFriends
  FROM people
*/

#[cfg(test)]
mod tests 
{

}