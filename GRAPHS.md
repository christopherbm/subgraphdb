# Graph Examples
- These are pulled from the Cypher documentation to work toward full compatibility.

```
CREATE
  (florentin:Person { name: 'Florentin', age: 16 }),
  (adam:Person { name: 'Adam', age: 18 }),
  (veselin:Person { name: 'Veselin', age: 20, ratings: [5.0] }),
  (hobbit:Book { name: 'The Hobbit', isbn: 1234, numberOfPages: 310, ratings: [1.0, 2.0, 3.0, 4.5] }),
  (frankenstein:Book { name: 'Frankenstein', isbn: 4242, price: 19.99 }),

  (florentin)-[:KNOWS { since: 2010 }]->(adam),
  (florentin)-[:KNOWS { since: 2018 }]->(veselin),
  (florentin)-[:READ { numberOfPages: 4 }]->(hobbit),
  (florentin)-[:READ { numberOfPages: 42 }]->(hobbit),
  (adam)-[:READ { numberOfPages: 30 }]->(hobbit),
  (veselin)-[:READ]->(frankenstein)
```

```
CREATE
  (alice:Developer {name:'Alice', age: 38, eyes: 'Brown'}),
  (bob:Administrator {name: 'Bob', age: 25, eyes: 'Blue'}),
  (charlie:Administrator {name: 'Charlie', age: 53, eyes: 'Green'}),
  (daniel:Adminstrator {name: 'Daniel', age: 54, eyes: 'Brown'}),
  (eskil:Designer {name: 'Eskil', age: 41, eyes: 'blue', likedColors: ['Pink', 'Yellow', 'Black']}),
  (alice)-[:KNOWS]->(bob),
  (alice)-[:KNOWS]->(charlie),
  (bob)-[:KNOWS]->(daniel),
  (charlie)-[:KNOWS]->(daniel),
  (bob)-[:MARRIED]->(eskil)
```

```
CREATE
  (keanu:Person {name:'Keanu Reeves', age:58, nationality:'Canadian'}),
  (carrie:Person {name:'Carrie Anne Moss', age:55, nationality:'American'}),
  (liam:Person {name:'Liam Neeson', age:70, nationality:'Northern Irish'}),
  (guy:Person {name:'Guy Pearce', age:55, nationality:'Australian'}),
  (kathryn:Person {name:'Kathryn Bigelow', age:71, nationality:'American'}),
  (jessica:Person {name:'Jessica Chastain', age:45, address:''}),
  (theMatrix:Movie {title:'The Matrix'}),
  (keanu)-[:KNOWS]->(carrie),
  (keanu)-[:KNOWS]->(liam),
  (keanu)-[:KNOWS]->(kathryn),
  (kathryn)-[:KNOWS]->(jessica),
  (carrie)-[:KNOWS]->(guy),
  (liam)-[:KNOWS]->(guy),
  (keanu)-[:ACTED_IN]->(theMatrix),
  (carrie)-[:ACTED_IN]->(theMatrix)
```

```
CREATE
  (alice:Developer {name:'Alice', age: 38, eyes: 'Brown'}),
  (bob:Administrator {name: 'Bob', age: 25, eyes: 'Blue'}),
  (charlie:Administrator {name: 'Charlie', age: 53, eyes: 'Green'}),
  (daniel:Adminstrator {name: 'Daniel', age: 54, eyes: 'Brown'}),
  (eskil:Designer {name: 'Eskil', age: 41, eyes: 'blue', likedColors: ['Pink', 'Yellow', 'Black']}),
  (alice)-[:KNOWS]->(bob),
  (alice)-[:KNOWS]->(charlie),
  (bob)-[:KNOWS]->(daniel),
  (charlie)-[:KNOWS]->(daniel),
  (bob)-[:MARRIED]->(eskil)
```

```
CREATE
  (copenhagen:TrainStation {latitude: 55.672874, longitude: 12.564590, city: 'Copenhagen'}),
  (malmo:Office {latitude: 55.611784, longitude: 12.994341, city: 'Malmö'}),
  (copenhagen)-[:TRAVEL_ROUTE]->(malmo)
```