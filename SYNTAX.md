# Syntax Examples

### SE 1
Returns all nodes in default graph.
```
MATCH ()
```


### SE 2
Creates given (simple) nodes and (simple) edges in the default graph. 
```
CREATE GRAPH devs
  (alice:Developer)
  (bob:Administrator)
  (chris:Lead)
  (alice)-[:KNOWS]-(bob)
  (alice)-[:KNOWS]-(chris)
  (bob)-[:KNOWS]-(chris)
```

### 
```
CREATE
  (keanu:Person)
  (carrie:Person)
  (liam:Person)
  (keanu)-[:KNOWS]->(carrie),
  (keanu)-[:KNOWS]->(liam)
```

```
MATCH (n:Developer)
MATCH (n { mode: 'Rail' })

MATCH (john:Person {name: 'John'})
MATCH (john)-[:FRIEND]->(friend)
RETURN friend.name AS friendName

MATCH (p:Person) FROM movies RETURN p LIMIT 5
MATCH (p:Person) RETURN p LIMIT 5
MATCH () FROM devs;
MATCH (n)
MATCH (n) FROM devs RETURN n.name
MATCH (n:Stop)

MATCH (n:(TrainStation & BusStation))
MATCH (n:(TrainStation | BusStation))
MATCH (n:(TrainStation & BusStation) | StationGroup)
MATCH (n:Station WHERE n.name STARTS WITH 'Preston') RETURN n
MATCH (n:Station WHERE n.name ENDS WITH 'Preston') RETURN n

MATCH (n:Developer)
FROM devs
RETURN n AS Developer
```
