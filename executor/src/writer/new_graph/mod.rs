use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use cmd::{ transaction::Transaction, EdgeStatement, NodeStatement };
use common::{ direction_to_str, DirectionType, LABEL_BYTES };
use datagramv2::grams::{ DGu64, Label, UUID };
use utils::{ cons_uuid, open_file };

use crate::core::CoreExecutor;
use crate::core_planner::WriteNewGraphPlanner;
use crate::writer::core::{ CoreWriteExecutor, PageWriteResult };

/* 
  WriteNewGraphExecutor
    :: new()
    :: execute()
        :: set_graph_name_uuid()
            :: find_graph_name()
        :: write()
            :: write_graph()
            :: write_data_page()
            :: write_node()
            :: write_edge()
            :: validate_edge_statement()
*/
pub struct WriteNewGraphExecutor<'a> 
{
  pub transaction: &'a Transaction,
  pub path: &'a str,
  pub page_size: usize,
  pub graph_uuid: Option<UUID>,
  pub graph_name: Option<Label>,
  pub err_state: Option<String>,
}

impl WriteNewGraphExecutor<'_>
{
  pub fn new<'a> ( t: &'a Transaction, path: &'a str, page_size: usize ) -> WriteNewGraphExecutor<'a>
  {
    WriteNewGraphExecutor 
    { 
      transaction: t, 
      path: path, 
      page_size: page_size,
      graph_uuid: None, 
      graph_name: None, 
      err_state: None 
    }
  }

  pub fn execute ( &mut self ) 
  {
    let name_res = self.set_graph_name_uuid();
    if name_res.is_ok() 
    {
      let graph_name = &self.graph_name.as_ref().unwrap().clone();
      let mut planner = WriteNewGraphPlanner::new( self.path.to_string(), graph_name );
      planner.plan();
      if planner.err_state == None { self.write( &planner ); }
      else 
      { 
        self.err_state = Some( planner.err_state.unwrap().to_string() ); 
        return;
      }
    }
    self.err_state = Some( String::from( "Error writing new graph." ));
  }

  pub fn write ( &mut self, planner: &WriteNewGraphPlanner ) 
  {
    let open_res = open_file( &PathBuf::from( &self.path ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );
    self.write_graph( planner, &mut writer );

    let mut curr_query_order: u16 = 1;
    let mut graph_order: u64 = 0;

    let page_write_result = self.write_data_page( planner, &mut writer ).unwrap();
    CoreExecutor::writer_seek_back_to( page_write_result.position_start_empty, &mut writer );
    loop 
    {
      if self.transaction.query_order > curr_query_order 
      {
        let node_stmt_opt = self.transaction.next_node_statement( curr_query_order );
        if node_stmt_opt.is_some() 
        {
          self.write_node( node_stmt_opt.unwrap(), graph_order, &mut writer );
          graph_order += 1;
          curr_query_order += 1;
          continue;
        }

        let edge_stmt_opt = self.transaction.next_edge_statement( curr_query_order );
        if edge_stmt_opt.is_some() 
        {
          let valid_uuids = self.validate_edge_statement( edge_stmt_opt.unwrap(), curr_query_order );
          if valid_uuids.is_some() 
          {
            self.write_edge( 
              &valid_uuids.as_ref().unwrap().0, 
              edge_stmt_opt.as_ref().unwrap(), 
              &valid_uuids.as_ref().unwrap().1,
              DirectionType::Undirected, // !!!
              graph_order,
              &mut writer );
            graph_order += 1;
            curr_query_order += 1;
            continue;
          }
        }
      }
      break;
    }
  }  
}

impl WriteNewGraphExecutor<'_>
{
  /// Write graph row to DBPage
  pub fn write_graph ( &mut self, planner: &WriteNewGraphPlanner, writer: &mut BufWriter<File> ) 
  {
    let res = CoreWriteExecutor::write_graph( 
      self.graph_uuid.as_ref().unwrap(), 
      self.graph_name.as_ref().unwrap(), 
      planner, 
      writer );
    if res.is_ok() { return; }
    self.err_state = Some( String::from( "Error writing new graph." ));
  }


  /// Write new DataPage to end of file
  pub fn write_data_page ( 
    &mut self, 
    planner: &WriteNewGraphPlanner, 
    writer: &mut BufWriter<File> ) -> Result<PageWriteResult, String> 
  {
    CoreWriteExecutor::write_data_page( 
      self.graph_uuid.as_ref().unwrap(), 
      self.graph_name.as_ref().unwrap(), 
      self.page_size, 
      planner, writer )
  }

  
  /// Write Node to current DataPage
  pub fn write_node ( &mut self, stmt: &NodeStatement, graph_order: u64, writer: &mut BufWriter<File> ) 
  {
    let uuid_res = UUID::new( stmt.id.clone() );
    let primary_label_res = Label::new( stmt.primary_label.clone(), LABEL_BYTES );
    CoreWriteExecutor::write_node( 
      &DGu64::new( graph_order ), 
      &uuid_res.unwrap(), 
      &primary_label_res.unwrap(), 
      writer );
  }
  
  
  /// Write Edge to current DataPage
  pub fn write_edge ( 
    &mut self, 
    left_uuid: &UUID, stmt: &EdgeStatement, right_uuid: &UUID, edge_dir: DirectionType, graph_order: u64, 
    writer: &mut BufWriter<File> ) 
  {
    let uuid_res = UUID::new( stmt.id.clone() );
    let primary_label_res = Label::new( stmt.primary_label.clone(), LABEL_BYTES );
    CoreWriteExecutor::write_edge ( 
      &DGu64::new( graph_order ), 
      &uuid_res.unwrap(), 
      &primary_label_res.unwrap(), 
      direction_to_str( &edge_dir ),
      left_uuid, 
      right_uuid, 
      writer )
  }


  pub fn set_graph_name_uuid ( &mut self ) -> Result<bool, String> 
  {
    let name_opt = self.find_graph_name();
    if name_opt.is_some() 
    {
      let res = Label::new( name_opt.unwrap(), LABEL_BYTES );
      if res.is_ok() { self.graph_name = Some( res.unwrap() ); }
      else { return Err( String::from( "Error finding graph name." )); }
    }

    let uuid_res = UUID::new( cons_uuid() );
    if uuid_res.is_ok() 
    {
      self.graph_uuid = Some( uuid_res.unwrap() );
      return Ok( true );
    }
    Err( String::from( "Error finding graph name." ))
  }
}

impl WriteNewGraphExecutor<'_> 
{

  /// Find new graph name within Transaction
  pub fn find_graph_name ( &self ) -> Option<String>
  {
    if self.transaction.create_statement.is_some() 
    {
      let stmt = self.transaction.create_statement.as_ref().unwrap();
      if stmt.graph_name.is_some() 
      {
        return Some( stmt.graph_name.as_ref().unwrap().to_string() )
      }
    }
    None
  }

  /// Validate EdgeStatement by checking NodeRefStatements and TransactionLabels
  pub fn validate_edge_statement ( &self, stmt: &EdgeStatement, curr_query_order: u16 ) -> Option<(UUID, UUID)> 
  {
    let left_res = self.transaction.next_ref_statement( curr_query_order - 1 );
    let right_res = self.transaction.next_ref_statement( curr_query_order + 1 );
    if left_res.is_some() && right_res.is_some() 
    {
      let left_node = self.transaction.find_node_by_transaction_label( &left_res.unwrap().transaction_label );
      let right_node = self.transaction.find_node_by_transaction_label( &right_res.unwrap().transaction_label );
      if left_node.is_some() && right_node.is_some() 
      {
        let left_uuid = UUID::new( left_node.unwrap().id.clone() );
        let right_uuid = UUID::new( right_node.unwrap().id.clone() );
        if left_uuid.is_ok() && right_uuid.is_ok() 
        {
          return Some(( left_uuid.unwrap(), right_uuid.unwrap() ));
        }
      }
    }
    None
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::fs::{ metadata, remove_file };
  use planner::process_query;
  use utils::create_file;
  use crate::writer::new_db::WriteNewDBExecutor;

  const PAGE_SIZE: usize = 4096;

  fn build_id () -> UUID { UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" )).unwrap() }
  fn db_nickname () -> Label { Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() }
  fn write_new_db ( path: &str )
  {
    let open_res = create_file( &PathBuf::from( path ));
    let mut stream = BufWriter::new( open_res.unwrap() );
    let _ = WriteNewDBExecutor::execute_write_new( &build_id(), &db_nickname(), PAGE_SIZE, &mut stream );
  }

  #[test]
  fn test_find_graph_name () 
  {
    let path_str = "../test_data/find_graph_name.sdb";    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let writer = WriteNewGraphExecutor::new( &t, path_str, 4096 );

    assert_eq!( writer.find_graph_name(), Some( String::from("devs")));
  }

  #[test]
  fn test_write_graph () 
  {
    let path_str = "../test_data/WriteNewGraphExecutor_test_write_graph.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let mut write_executor = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    
    write_executor.graph_name = Some( Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() );
    write_executor.graph_uuid = Some( UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" )).unwrap() );

    let name_label = Label::new( String::from( "devs2" ), LABEL_BYTES );
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), name_label.as_ref().unwrap() );
    planner.plan();

    let open_res = open_file( &PathBuf::from( &path_str ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );

    write_executor.write_graph( &planner, &mut writer );

    assert_eq!( write_executor.err_state, None );

    let _ = remove_file( PathBuf::from( path_str ));
  }
  
  #[test]
  fn test_write_data_page () 
  {
    let path_str = "../test_data/WriteNewGraphExecutor_test_write_data_page.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let mut write_executor = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    
    write_executor.graph_name = Some( Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() );
    write_executor.graph_uuid = Some( UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" )).unwrap() );

    let name_label = Label::new( String::from( "devs2" ), LABEL_BYTES );
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), name_label.as_ref().unwrap() );
    planner.plan();

    let open_res = open_file( &PathBuf::from( &path_str ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );

    let res = write_executor.write_data_page( &planner, &mut writer );

    assert_eq!( write_executor.err_state, None );    
    assert_eq!( res.is_ok(), true );
    assert_eq!( res.as_ref().unwrap().empty_cell_count, 494 );
    assert_eq!( res.as_ref().unwrap().position_start_empty, 4224 );
    assert_eq!( res.as_ref().unwrap().err_state, None );

    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_write_node () 
  {
    let path_str = "../test_data/WriteNewGraphExecutor_test_write_node.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let mut write_executor = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    
    write_executor.graph_name = Some( Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() );
    write_executor.graph_uuid = Some( UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" )).unwrap() );

    let stmt = NodeStatement::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0cX" ), 
      0, 
      None, 
      String::from( "primary" ));

    let open_res = open_file( &PathBuf::from( &path_str ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );

    let _ = write_executor.write_node( &stmt, 0, &mut writer );

    assert_eq!( write_executor.err_state, None );

    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_write_edge () 
  {
    let path_str = "../test_data/WriteNewGraphExecutor_test_write_edge.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let mut write_executor = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    
    write_executor.graph_name = Some( Label::new( String::from( "devs" ), LABEL_BYTES ).unwrap() );
    write_executor.graph_uuid = Some( UUID::new( String::from( cons_uuid() )).unwrap() );

    let stmt = EdgeStatement::new( String::from( cons_uuid() ), 0, None, String::from( "primary" ));

    let open_res = open_file( &PathBuf::from( &path_str ));
    let mut writer: BufWriter<File> = BufWriter::new( open_res.unwrap() );

    let _ = write_executor.write_edge( 
      &UUID::generate(), 
      &stmt, 
      &UUID::generate(),
      DirectionType::Undirected,
      0,
      &mut writer );

    assert_eq!( write_executor.err_state, None );

    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_validate_edge_statement () 
  {
    let path_str = "../test_data/WriteNewGraphExecutor_test_validate_edge_statement.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer)
        (bob:Administrator)
        (chris:Lead)
        (alice)-[:KNOWS]-(bob)
        (alice)-[:KNOWS]-(chris)
        (bob)-[:KNOWS]-(chris)
    ";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let writer = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    let res = writer.validate_edge_statement( t.edge_statements.get( 0 ).as_ref().unwrap(), 5 );

    assert_eq!( res.is_some(), true );
    assert_eq!( res.as_ref().unwrap().0.val, t.node_statements.get( 0 ).as_ref().unwrap().id );
    assert_eq!( res.as_ref().unwrap().1.val, t.node_statements.get( 1 ).as_ref().unwrap().id );
    
    let _ = remove_file( PathBuf::from( path_str ));
  }

  // -------------------------------------------------------------------------------------------------------------------
  #[test]
  fn test_create_graph_2 () 
  {
    let path_str = "../test_data/test_create_graph_2.sdb";
    write_new_db( path_str );
    
    let query_string = "
      CREATE GRAPH devs
        (alice:Developer)
        (bob:Administrator)
        (chris:Lead)
        (alice)-[:KNOWS]-(bob)
        (alice)-[:KNOWS]-(chris)
        (bob)-[:KNOWS]-(chris)
    ";
    let t = process_query( &query_string, build_id(), db_nickname() );
    println!( "{}", t );
    //let mut writer = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    


    //let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_create_graph_1 () 
  {
    let path_str = "../test_data/create_graph_1.sdb";
    write_new_db( path_str );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );
    
    let query_string = "CREATE GRAPH devs";
    let t = process_query( &query_string, build_id(), db_nickname() );
    let mut writer = WriteNewGraphExecutor::new( &t, path_str, 4096 );
    writer.execute();

    assert_eq!( writer.err_state, None );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), 4096 as u64 );

    let name_label = Label::new( String::from( "devs2" ), LABEL_BYTES );
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), name_label.as_ref().unwrap() );
    planner.plan();

    assert_eq!( planner.db_page.is_some(), true );
    assert_eq!( planner.db_page.as_ref().unwrap().start_pos, 0 );
    assert_eq!( planner.db_page.as_ref().unwrap().end_pos, Some( 4088 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_count, Some( 461 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_start_pos, Some( 384 ));

    let _ = remove_file( PathBuf::from( path_str ));
  }
}