/*
CREATE movies
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


<Transaction>
  <DB is known and valid for this example. />
  <CreateGraph uuid 0 name="movies" />
  <InsertNode uuid 0 (keanu) Person IndexedProps {...} />
  <InsertNode uuid 1 (carrie) Person IndexedProps {...} />
  <InsertNode uuid 2 (liam) Person IndexedProps {...} />
  <InsertNode uuid 3 (guy) Person IndexedProps {...} />
  <InsertNode uuid 4 (kathryn) Person IndexedProps {...} />
  <InsertNode uuid 5 (jessica) Person IndexedProps {...} />
  <InsertNode uuid 6 (theMatrix) Movie IndexedProps {...} />

  <InsertEdge uuid 0 (keanu) KNOWS (carrie) />
  <InsertEdge uuid 1 (keanu) KNOWS (liam) />
  <InsertEdge uuid 2 (keanu) KNOWS (kathryn) />
  <InsertEdge uuid 3 (kathryn) KNOWS (jessica) />
  <InsertEdge uuid 4 (carrie) KNOWS (guy) />
  <InsertEdge uuid 5 (liam) KNOWS (guy) />
  <InsertEdge uuid 6 (keanu) ACTED-IN (theMatrix) />
  <InsertEdge uuid 7 (carrie) ACTED-IN (theMatrix) />
  
  <InsertNodeUnindexedProps {...} />
</Transaction>



----------------

- Certain actions will force a checkpoint. These should be isolated, but still part of a transaction.

MATCH (p:Person) RETURN p LIMIT 5

*/

pub fn add(left: u64, right: u64) -> u64 
{
  left + right
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn it_works () 
  {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
