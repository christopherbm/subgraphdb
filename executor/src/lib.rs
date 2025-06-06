use std::path::PathBuf;
use std::io::{ BufWriter, Write };
use std::fs::{ File };
use utils::{ cons_uuid, create_file, open_file, path_exists, process_str };
use cmd::{ Transaction, CreateGraph, CreateNode, CreateEdge, CreateNodeRef, MatchClause, FromClause };
use datagramv2::rows::{ BuildIDRow, DBNicknameRow, GraphRow, cons_node_row, cons_edge_row };
use datagramv2::dg_utils::{ next_row_affix };
use datagramv2::constants::{ DataGramType };
use common::{ DirectionType, DIR_UNDIRECTED, DIR_LEFT, DIR_RIGHT, DIR_BI, direction_to_str, LABEL_BYTES };

/*
// !! page size needs to be configurable, but row size will be fixed. Page size cannot be changed after db creation.
// !! there can be more than 1 sdb config page depending on number and complexity of graphs
// !! knows that "name" is first of indexed values - this would need to be part of graph config as a contraint
// !! any modification to the graph config or the db config will result in a rewrite of the binary data

- When a read begins, it chooses a particular transaction id to be its "end mark". This way reads can happen along with
  writes but the reads will have "point-in-time" consistency.
*/

pub struct Executor
{
  pub path: String,
  pub t: Transaction,

  pub write_new: bool,
}
impl Executor
{
  pub fn new ( path: String, t: Transaction ) -> Result<Executor, String> 
  {
    Ok( Executor { path: path, t: t, write_new: false })
    //Err( String::from( "Executor Error." ))
  }

  pub fn execute ( &mut self ) 
  {
    if self.t.has_writes() 
    {
      self.create_or_open();
      self.write();
    }
    else 
    {
      self.read();
    }
  }

  pub fn write ( &mut self ) 
  {
    let open_res = create_file( &PathBuf::from( &self.path )); // create opens in write mode
    if open_res.is_ok() 
    {
      let mut stream = BufWriter::new( open_res.unwrap() );
      if self.write_new == true 
      { 
        WriteExecutor::execute_write_new( &self.t.build_id, &self.t.db_nickname, &mut stream ); 
      }
      
      if self.t.create_graph.is_some() 
      { 
        let create_row = self.t.create_graph.as_ref().unwrap();
        WriteExecutor::execute_create_graph( &create_row.id, &create_row.name, &mut stream ); 
      }

      let graph_id_res = ReadExecutor::find_graph_id( &self.t );
      if graph_id_res.is_some() 
      {
        if self.t.create_node.len() > 0 
        {
          WriteExecutor::execute_write_node( graph_id_res.as_ref().unwrap(), &self.t.create_node, &mut stream );
        }
        if self.t.create_edge.len() > 0 
        {
          WriteExecutor::execute_write_edge( graph_id_res.as_ref().unwrap(), &self.t.create_edge, &mut stream );
        }
      }
      stream.flush().unwrap();
    }
  }

  pub fn read ( &mut self ) 
  {
    let open_res = open_file( &PathBuf::from( &self.path ));
    if open_res.is_ok() 
    {
      let f: &mut File = &mut open_res.unwrap();
      let mut reader = ReadExecutorV2::new( &self.t, f );
      reader.analyze_transaction();
      println!( "{:?}", reader.next() );
      //ReadExecutor::execute_match( &self.t, f );
    }
  }

  pub fn create_or_open ( &mut self ) 
  {
    let exists_res = path_exists( PathBuf::from( &self.path ));
    if exists_res.is_ok() && exists_res.unwrap() == true
    {}
    else 
    {
      let create_res = create_file( &PathBuf::from( &self.path ));
      if create_res.is_ok() { self.write_new = true; }
    }
  }  
}

pub struct WriteExecutor {}
impl WriteExecutor 
{
  pub fn execute_write_node ( graph_id: &str, nodes: &Vec<CreateNode>, stream: &mut impl Write ) 
  {
    for cn in nodes.iter() 
    {
      let _ = stream.write( &cons_node_row( graph_id, &cn.id, &cn.primary_label )).unwrap();
    }
  }

  pub fn execute_write_edge ( graph_id: &str, edges: &Vec<CreateEdge>, stream: &mut impl Write ) 
  {
    for eg in edges.iter() 
    {
      let _ = stream.write( &cons_edge_row( 
        graph_id, 
        &eg.id, 
        &eg.primary_label, 
        direction_to_str( &eg.edge_dir ), 
        &eg.left_ref.id, 
        &eg.right_ref.id )).unwrap();
    }
  }

  /// Create a new db file
  pub fn execute_write_new ( build_id: &str, db_nickname: &str, stream: &mut impl Write ) 
  {
    let _ = stream.write( &BuildIDRow::new( build_id )).unwrap();
    let _ = stream.write( &DBNicknameRow::new( db_nickname )).unwrap();
    let str_res = process_str( LABEL_BYTES, String::from( "DEFAULT_GRAPH" ));
    if str_res.is_ok() 
    {
      let default_graph_uuid = "1b622a2c-68dc-4848-a018-e71b604b5597::::";
      let _ = stream.write( &GraphRow::new( default_graph_uuid, str_res.as_ref().unwrap() )).unwrap();
    }
  }

  pub fn execute_create_graph ( id: &str, name: &str, stream: &mut impl Write ) 
  {
    let _ = stream.write( &GraphRow::new( id, &name )).unwrap();
  }
}

pub struct ReadExecutorV2<'a> 
{ 
  pub t: &'a Transaction, 
  pub read_state: Option<MatchTransactionState<'a>>,
  pub f: &'a mut File, 
  pub last_read: DataGramType 
}
impl ReadExecutorV2<'_>
{
  pub fn new<'a> ( t: &'a Transaction, f: &'a mut File ) -> ReadExecutorV2<'a>
  {
    ReadExecutorV2 { t: t, read_state: None, f: f, last_read: DataGramType::UnStarted }
  }

  pub fn analyze_transaction ( &mut self ) 
  {
    if self.t.from_clause.is_some() 
    {
      self.read_state = 
        Some( 
          MatchTransactionState::new( 
            Some( &self.t.from_clause.as_ref().unwrap().graph_name )));
    }
  }
}
impl Iterator for ReadExecutorV2<'_>
{
  type Item = String;

  fn next( &mut self ) -> Option<Self::Item> 
  {
    println!( "{:?}", self.read_state );
    None
  }
}

pub struct ReadExecutor {}
impl ReadExecutor 
{
  pub fn find_graph_id ( t: &Transaction ) -> Option<&str>
  {
    if t.create_graph.is_some() 
    {
      return Some( &t.create_graph.as_ref().unwrap().id );
    }
    None
  }

  pub fn execute_match( t: &Transaction, f: &mut File ) 
  {
    let match_clause: &MatchClause = t.match_clause.as_ref().unwrap();    
    if t.from_clause.is_some() 
    {
      let from_clause: &FromClause = t.from_clause.as_ref().unwrap();
      let graph_res = ReadExecutor::find_graph_by_name( &from_clause.graph_name, f );

      println!( "{:?}", graph_res );
    }
  }

  // assumes beginning of stream
  pub fn find_graph_by_name ( name: &str, f: &mut File ) -> Option<( String, String )>
  {
    if BuildIDRow::is_affix( &next_row_affix( f ).unwrap_or( String::from( "" ))) 
    { 
      let _ = BuildIDRow::skip( f ); 
    }
    
    if DBNicknameRow::is_affix( &next_row_affix( f ).unwrap_or( String::from( "" ))) 
    { 
      let _ = DBNicknameRow::skip( f ); 
    }

    // !!! while loop

    if GraphRow::is_affix( &next_row_affix( f ).unwrap_or( String::from( "" ))) 
    {
      let graph_res = GraphRow::read( f );
      if graph_res.is_ok() 
      {
        if graph_res.as_ref().unwrap().1 == name 
        {
          return Some( graph_res.unwrap() );
        }
      }
    }

    None
  }
}


// transaction states
#[derive( Debug )]
pub struct MatchTransactionState<'a>
{
  pub graph_name: Option<&'a str>,
  pub graph_id: Option<String>,
}
impl MatchTransactionState<'_>
{
  pub fn new ( graph_name: Option<&str> ) -> MatchTransactionState
  {
    MatchTransactionState { graph_name: graph_name, graph_id: None }
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::io::Read;
  use std::fs::{ remove_file, metadata };
  use planner::{ process_query };

  #[test]
  fn test_se1_logic () 
  {
    let path_str = "../test_data/tmp_se1a.sdb";
    let open_res = create_file( &PathBuf::from( &path_str ));
    let mut stream = BufWriter::new( open_res.unwrap() );
    let build_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let db_nickname = String::from( "devs\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" );

    let _ = WriteExecutor::execute_write_new ( &build_id, &db_nickname, &mut stream );

    //let _ = remove_file( PathBuf::from( path_str ));
  }

  /* 
  #[test]
  fn test_execute_transaction_1 () 
  {
    //let page_size: usize = 4096;
    let path_str = "../test_data/tmp_db1.sdb";

    let query_string = "CREATE GRAPH devs";
    let t:Transaction = process_query( query_string );

    let mut executor_res = Executor::new( path_str.to_string(), t );
    assert_eq!( executor_res.is_ok(), true );

    let executor = executor_res.as_mut().unwrap();
    executor.execute();

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 256 );
    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_execute_transaction_2 () 
  {
    //let page_size: usize = 4096;
    let path_str = "../test_data/tmp_db2.sdb";

    let query_string = "
      CREATE GRAPH devs
        (alice:Developer),
        (bob:Administrator),
        (alice)-[:KNOWS]->(bob)
    ";
    let t:Transaction = process_query( query_string );

    let mut executor_res = Executor::new( path_str.to_string(), t );
    assert_eq!( executor_res.is_ok(), true );

    let executor = executor_res.as_mut().unwrap();
    executor.execute();

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 824 );
    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_execute_transaction_3 () 
  {
    //let path_str = "../test_data/tmp_db3.sdb";
    let _query_string = "
      MATCH (n:Developer)
      FROM devs
      RETURN n AS Developer
    ";
    //let t:Transaction = process_query( query_string );
    //println!( "{}", t );

    //let mut executor_res = Executor::new( path_str.to_string(), t );
    //assert_eq!( executor_res.is_ok(), true );

    //let executor = executor_res.as_mut().unwrap();
    //executor.execute();

    //assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 824 );
    //let _ = remove_file( PathBuf::from( path_str ));
  }

  */

  #[test]
  fn test_execute_transaction_4 () 
  {
   // let path_str = "../test_data/tmp_db4.sdb";
    /*
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer),
        (bob:Administrator),
        (alice)-[:KNOWS]->(bob)
    ";
    let t:Transaction = process_query( query_string );
    let mut executor_res = Executor::new( path_str.to_string(), t );
    assert_eq!( executor_res.is_ok(), true );
    let _ = executor_res.as_mut().unwrap().execute();
    */

    // ---
    //println!( "--------------------------------" );
    //let t1:Transaction = process_query( "MATCH () FROM devs" );
    //println!( "{}", t1 );
    //let mut executor_res1 = Executor::new( path_str.to_string(), t1 );
    //assert_eq!( executor_res1.is_ok(), true );
    //let _ = executor_res1.as_mut().unwrap().execute();
    //println!( "--------------------------------" );

    // ---
    //let t2:Transaction = process_query( "MATCH (n) FROM devs" );
    //let mut executor_res2 = Executor::new( path_str.to_string(), t2 );
    //assert_eq!( executor_res2.is_ok(), true );
    //let _ = executor_res2.as_mut().unwrap().execute();

    //assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 824 );
    //let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_errors () 
  {
    let path_str = "../test_data/empty.sdb";
    let open_res = open_file( &PathBuf::from( path_str ));
    assert_eq!( open_res.is_ok(), true );
    
    let f: &mut File = &mut open_res.unwrap();
    assert_eq!( next_row_affix( f ).is_none(), true );
    
    //println!( "{:?}",  );
  } 

  #[test]
  fn test_adjacency_matrix () 
  {
    //let page_size: usize = 4096;
    let path_str = "../test_data/tmp_db3.sdb";

    let query_string = "
      CREATE GRAPH adj_matrix
      (w:Node {name: 'w', group: 1}),
      (x:Node {name: 'x', group: 2}),
      (y:Node {name: 'y', group: 1}),
      (z:Node {name: 'z', group: 2}),


      /// default connections
      (w)-->(y),
      (w)<-->(z),
      (w)<--(x),
    ";

    let query_string = "
      INSERT INTO adj_matrix
      (a:Node {name: 'a', group: 1}),
      (b:Node {name: 'b', group: 3}),
      /// How to match to existing nodes?
    ";
    
    //let t:Transaction = process_query( query_string );
    //println!( "{}", t );

    //let mut executor_res = Executor::new( path_str.to_string(), t );
    //assert_eq!( executor_res.is_ok(), true );

    //let executor = executor_res.as_mut().unwrap();
    //executor.execute();

    //assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 824 );
    //let _ = remove_file( PathBuf::from( path_str ));
  }
}
