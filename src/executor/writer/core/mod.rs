use std::fs::{ File };
use std::io::{ BufWriter, Seek, SeekFrom, Write };
use crate::common::{ END_DB, LABEL_BYTES, PLACEHOLDER };
use crate::datagramv2::internal_grams::{ DGu64, Label, UUID };
use crate::datagramv2::rows::{ BuildIDRow, DBNicknameRow, EdgeRow, GraphRow, NodeRow, PageRow };
use crate::executor::core_planner::{ EmptySpace, WriteNewGraphPlanner };


#[derive(Debug)]
pub struct PageWriteResult 
{
  pub empty_cell_count: usize,
  pub position_start_empty: u64,
  pub err_state: Option<String>
}

impl PageWriteResult 
{
  pub fn new ( empty_cell_count: usize, position_start_empty: u64, err_state: Option<String> ) -> PageWriteResult 
  {
    PageWriteResult 
    { 
      empty_cell_count: empty_cell_count, 
      position_start_empty: position_start_empty, 
      err_state: err_state 
    }
  }
}

pub struct CoreWriteExecutor {}
impl CoreWriteExecutor
{
  /// Create a new db file
  pub fn write_new_db ( 
    build_id: &UUID, db_nickname: &Label, page_size: usize, 
    stream: &mut impl Write ) 
  {
    let byte_size: usize = 280; // bytes consumed by written data
    let _ = stream.write( &PageRow::new_db_affix() ).unwrap();
    let _ = stream.write( &BuildIDRow::new( build_id )).unwrap();
    let _ = stream.write( &DBNicknameRow::new( db_nickname )).unwrap();

    let default_graph_uuid = UUID::new( String::from( "1b622a2c-68dc-4848-a018-e71b604b5597" ));
    let default_graph_name = Label::new( String::from( "DEFAULT_GRAPH" ) );
    let _ = stream.write( &GraphRow::new( &default_graph_uuid.unwrap(), &default_graph_name.unwrap() )).unwrap();
    
    let _ = stream.write( &PageRow::gen_empty_cells( page_size - byte_size )).unwrap();

    let _ = stream.write( &PageRow::new_db_affix() ).unwrap();
    let _ = stream.write( &END_DB.as_bytes() ).unwrap();
  }

  
  /// Writes a new graph to a DBPage
  pub fn write_graph ( 
    graph_uuid: &UUID, graph_name: &Label, 
    planner: &WriteNewGraphPlanner, writer: &mut BufWriter<File> ) -> Result<bool, String>
  {
    let db_space: EmptySpace = planner.db_page_has_space( GraphRow::cell_count() );
    if db_space != EmptySpace::NotEnough 
    {
      let _ = writer.seek( 
        SeekFrom::Current( 
          planner.db_page.as_ref().unwrap().empty_cell_start_pos.unwrap() as i64 ));
      let _ = writer.write( &GraphRow::new( graph_uuid, graph_name ) ).unwrap();

      if db_space == EmptySpace::HasEnough 
      {
        let _ = writer.write( &PageRow::new_start_empty_affix() ).unwrap();
      }
      return Ok( true );
    }
    Err( String::from( "Error writing new graph." ))
  }

  
  /// write new data page to end of file
  /// returns (number of empty cells, position of first empty cell) ( usize, u64 )
  pub fn write_data_page (
    graph_uuid: &UUID, graph_name: &Label, page_size: usize,
    planner: &WriteNewGraphPlanner, writer: &mut BufWriter<File> ) -> Result<PageWriteResult, String> 
  {
    let seek_end_res = CoreWriteExecutor::seek_end_affix( planner, writer );
    if seek_end_res.is_err() { return Err( seek_end_res.unwrap_err() ); }

    let _ = writer.write( &PLACEHOLDER.as_bytes() ).unwrap();
    let _ = writer.write( &PageRow::new_data_affix() ).unwrap();
    let _ = writer.write( &GraphRow::new( graph_uuid, graph_name ) ).unwrap();
    let _ = writer.write( &PageRow::gen_empty_cells( page_size - PageRow::data_page_size() )).unwrap();
    let _ = writer.write( &PageRow::new_data_affix() ).unwrap();
    let _ = writer.write( &END_DB.as_bytes() ).unwrap(); 

    let start_empty_pos = seek_end_res.unwrap() + (PageRow::data_page_size() as u64) - 8;
    Ok( PageWriteResult::new( 
      PageRow::empty_cell_count( page_size - PageRow::data_page_size() ), 
      start_empty_pos, 
      None ))
  }


  /// Write new node to data page (!!! handle errors)
  pub fn write_node ( 
    graph_order: &DGu64, node_id: &UUID, primary_label: &Label, 
    writer: &mut BufWriter<File> )
  {
    let _ = writer.write( &NodeRow::new( graph_order, node_id, primary_label ) ).unwrap();
  }


  /// Write new edge to data page (!!! handle errors)
  pub fn write_edge ( 
    graph_order: &DGu64, edge_id: &UUID, primary_label: &Label, 
    edge_dir: &str, left_uuid: &UUID, right_uuid: &UUID, 
    writer: &mut BufWriter<File> )
  {
    let _ = writer.write( &EdgeRow::new( 
      graph_order, 
      edge_id, 
      primary_label,
      edge_dir,
      left_uuid,
      right_uuid )).unwrap();
  }


  pub fn write_properties ( planner: &WriteNewGraphPlanner, writer: &mut BufWriter<File> ) -> Result<bool, String> 
  {
    Err( String::from( "Error writing properties" ))
  }

  
  /// Write a Label
  pub fn write_label ( label: &Label, writer: &mut BufWriter<File> ) 
  {
    let _ = writer.write( &label.unwrap().into_bytes() ).unwrap();
  }

  
  // Write a UUID
  pub fn write_uuid ( uuid: &UUID, writer: &mut BufWriter<File> ) 
  {
    let _ = writer.write( &uuid.unwrap().into_bytes() ).unwrap();
  }
}

impl CoreWriteExecutor 
{
  // move stream to start of end affix (so new data can be added)
  pub fn seek_end_affix ( planner: &WriteNewGraphPlanner, writer: &mut BufWriter<File> ) -> Result<u64, String>
  {
    if planner.end_pos.is_some() 
    {
      let seek_res = writer.seek( SeekFrom::Current(( planner.end_pos.as_ref().unwrap() - 8 ) as i64 ));
      if seek_res.is_ok() { return Ok( seek_res.unwrap() ); }
      else { return Err( seek_res.unwrap_err().to_string() ); }
    }
    Err( String::from( "Error seeking end of file." ))
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::fs::{ metadata, remove_file };
  use std::path::{ PathBuf };
  use crate::common::{ direction_to_str, DirectionType };
  use crate::utils::{ cons_uuid, create_file, open_file };
  use crate::executor::core::CoreExecutor;

  const PAGE_SIZE: usize = 4096;

  fn build_id () -> UUID { UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" )).unwrap() }
  fn db_nickname () -> Label { Label::new( String::from( "devs" ) ).unwrap() }

  fn write_new_db ( path: &str )
  {
    let open_res = create_file( &PathBuf::from( path ));
    let mut stream = BufWriter::new( open_res.unwrap() );
    let _ = CoreWriteExecutor::write_new_db( &build_id(), &db_nickname(), PAGE_SIZE, &mut stream );
  }

  #[test]
  fn test_write_new_db ()
  {
    let path_str = "../test_data/new.sdb";
    //let open_res = create_file( &PathBuf::from( path_str ));
    //let mut stream = BufWriter::new( open_res.unwrap() );
    //let _ = CoreWriteExecutor::write_new_db( &build_id(), &db_nickname(), page_size, &mut stream );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), PAGE_SIZE as u64 );
  }

  #[test]
  fn test_write_graph ()
  {
    let path_str = "../test_data/test_write_graph.sdb";
    write_new_db( path_str );

    let binding = db_nickname();
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), &binding );
    planner.plan();

    let new_uuid = UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c2" )).unwrap();
    let new_name = Label::new( String::from( "devs" ) ).unwrap();
    let open_res = open_file( &PathBuf::from( path_str ));
    let mut writer = BufWriter::new( open_res.unwrap() );
    let _ = CoreWriteExecutor::write_graph( &new_uuid, &new_name, &planner, &mut writer );

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), PAGE_SIZE as u64 );
    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_write_data_page ()
  {
    //let path_str = "../test_data/new_with_data_page.sdb";
    let path_str = "../test_data/test_write_data_page.sdb";
    write_new_db( path_str );

    let binding = db_nickname();
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), &binding );
    planner.plan();

    let new_uuid = UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c2" )).unwrap();
    let new_name = Label::new( String::from( "devs" ) ).unwrap();
    let open_res = open_file( &PathBuf::from( path_str ));
    let mut writer = BufWriter::new( open_res.unwrap() );
    let wdp_res = CoreWriteExecutor::write_data_page( &new_uuid, &new_name, PAGE_SIZE, &planner, &mut writer );

    assert_eq!( wdp_res.as_ref().unwrap().empty_cell_count, 494 );
    assert_eq!( wdp_res.as_ref().unwrap().position_start_empty, 4224 );
    assert_eq!( wdp_res.as_ref().unwrap().err_state, None );

    let _ = writer.flush();

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), (PAGE_SIZE * 2) as u64 );
    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_write_node ()
  {
    let path_str = "../test_data/test_write_node.sdb";
    write_new_db( path_str );

    let binding = db_nickname();
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), &binding );
    planner.plan();

    let new_uuid = UUID::new( String::from( cons_uuid() )).unwrap();
    let new_name = Label::new( String::from( "devs" ) ).unwrap();
    let open_res = open_file( &PathBuf::from( path_str ));
    let mut writer = BufWriter::new( open_res.unwrap() );
    let wdp_res = CoreWriteExecutor::write_data_page( &new_uuid, &new_name, PAGE_SIZE, &planner, &mut writer );

    assert_eq!( wdp_res.as_ref().unwrap().empty_cell_count, 494 );
    assert_eq!( wdp_res.as_ref().unwrap().position_start_empty, 4224 );
    assert_eq!( wdp_res.as_ref().unwrap().err_state, None );

    CoreExecutor::writer_seek_back_to( wdp_res.unwrap().position_start_empty , &mut writer );
    
    let graph_order = DGu64::new( 0 );
    let node_id = UUID::new( String::from( cons_uuid() )).unwrap();
    let primary_label = Label::new( String::from( "node" ) ).unwrap();
    CoreWriteExecutor::write_node( &graph_order, &node_id, &primary_label, &mut writer );
    let _ = writer.write( &PageRow::new_start_empty_affix() ).unwrap();

    let _ = writer.flush();

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), (PAGE_SIZE * 2) as u64 );
    let _ = remove_file( PathBuf::from( path_str ));
  }

  #[test]
  fn test_write_edge ()
  {
    let path_str = "../test_data/test_write_edge.sdb";
    write_new_db( path_str );

    let binding = db_nickname();
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), &binding );
    planner.plan();

    let new_uuid = UUID::new( String::from( cons_uuid() )).unwrap();
    let new_name = Label::new( String::from( "devs" ) ).unwrap();
    let open_res = open_file( &PathBuf::from( path_str ));
    let mut writer = BufWriter::new( open_res.unwrap() );
    let wdp_res = CoreWriteExecutor::write_data_page( &new_uuid, &new_name, PAGE_SIZE, &planner, &mut writer );

    assert_eq!( wdp_res.as_ref().unwrap().empty_cell_count, 494 );
    assert_eq!( wdp_res.as_ref().unwrap().position_start_empty, 4224 );
    assert_eq!( wdp_res.as_ref().unwrap().err_state, None );

    CoreExecutor::writer_seek_back_to( wdp_res.unwrap().position_start_empty , &mut writer );
    
    let graph_order = DGu64::new( 0 );
    let edge_id = UUID::new( String::from( cons_uuid() )).unwrap();
    let primary_label = Label::new( String::from( "edge" ) ).unwrap();
    let edge_dir = direction_to_str( &DirectionType::Undirected );
    let left_id = UUID::new( String::from( cons_uuid() )).unwrap();
    let right_id = UUID::new( String::from( cons_uuid() )).unwrap();
    CoreWriteExecutor::write_edge( 
      &graph_order, &edge_id, &primary_label, 
      edge_dir, &left_id, &right_id, 
      &mut writer );
    let _ = writer.write( &PageRow::new_start_empty_affix() ).unwrap();

    let _ = writer.flush();

    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), (PAGE_SIZE * 2) as u64 );
    let _ = remove_file( PathBuf::from( path_str ));
  }
}