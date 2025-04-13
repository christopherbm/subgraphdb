/*
00 -> [PageStart name:"db config" end_index:0A][wc]
01 -> [BuildUUID][db_nickname][wc]
.. -> [Graph 0 nickname:"movies" start_index:0B][wc]
0A -> [PageEnd "dbconfig"][wc]
0B -> [PageStart name:"graph 0 page" end_index:0C next_page_index:00][wc]
.. -> [node_order:0][UUID][Person]["Keanu Reaves"][node_conns:1 2 4][edge_cons:0 1 2][WC]
.. -> [node_order:1][UUID][Person]["Carrie Anne Moss"][node_conns:0 3][edge_cons:0 4][WC]
0C -> [PageEnd name:"graph 0 page"][wc]

// ---------------

// !! knows that "name" is first of indexed values - this would need to be part of graph config as a contraint
// !! any modification to the graph config or the db config will result in a rewrite of the binary data


// ---------------
MATCH (p:Person) FROM movies RETURN p LIMIT 5

*/

/// Executes a Transaction
pub fn execute_transaction () {}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_execute_transaction () 
  {
    assert_eq!( true, true );
  }
}
