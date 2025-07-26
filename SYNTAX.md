# Syntax Examples

### SE 1
Returns all nodes in default graph.
```
MATCH ()

MATCH (n)
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


### SE 3
Returns all nodes from given graph.
```
MATCH () FROM devs;
```


### SE 4
Match all nodes from given graph but return only given property.
```
MATCH (n) FROM devs RETURN n.name
```


### SE 5
Match all nodes from default graph with the given primary label.
```
MATCH (n:Stop)

MATCH (n:Developer)
```


### SE 6
Match all nodes from given graph with the given primary label. Return with custom label.
```
MATCH (n:Developer)
FROM devs
RETURN n AS Developer
```


### SE 7
Match a specific node (or nodes) from default graph. Set new property value.
```
MATCH (p:Person {name: 'Jennifer'})
SET p.birthdate = date('1980-01-01')
RETURN p
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
