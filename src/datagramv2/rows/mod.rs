use std::io::{ Seek, SeekFrom, Read, Error };
use std::fs::{ File };
use crate::utils::{ str_from_bytes };
use crate::common::{ END_DB, LABEL_BYTES, PLACEHOLDER, ROW_AFFIX_BYTES, UUID_BYTES };
use crate::datagramv2::dg_utils::next_u64;
use crate::datagramv2::grams::{ DGu64, Label, UUID };

// !! ALL VALUES MUST BE PADDED !!

pub struct BuildIDRow {}
impl BuildIDRow 
{
  const AFFIX: &'static str = "[::::BI]";

  pub fn new ( build_id: &UUID ) -> Vec<u8> 
  {
    let mut ret: String = String::from( "" );
    ret.push_str( BuildIDRow::AFFIX );    // [::::BI]
    ret.push_str( &build_id.unwrap() );   // [UUID]
    ret.push_str( BuildIDRow::AFFIX );    // [::::BI]
    ret.into_bytes()
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == BuildIDRow::AFFIX { return true; }
    false
  }

  /// Assumes first affix has been read
  pub fn read ( f: &mut File ) -> Result<String, String> 
  {
    let mut buffer = [ 0; UUID_BYTES ];
    let _ = f.read_exact( &mut buffer );
    let uuid_res = str_from_bytes( &buffer.to_vec() );
    if uuid_res.is_err() { return Err( String::from( "Read Error: Build ID" ));}
    let _ = f.seek( SeekFrom::Current(( ROW_AFFIX_BYTES ) as i64 ));
    Ok( uuid_res.unwrap() )
  }

  /// Assumes first affix has been read
  pub fn skip ( f: &mut File ) -> Result<u64, Error>
  {
    return f.seek( SeekFrom::Current(( UUID_BYTES + ROW_AFFIX_BYTES ) as i64 ));
  }
}

pub struct DBNicknameRow {}
impl DBNicknameRow 
{
  const AFFIX: &'static str = "[::DBNN]";

  pub fn new ( nickname: &Label ) -> Vec<u8> 
  {
    let mut ret: String = String::from( "" );
    ret.push_str( DBNicknameRow::AFFIX );   // [::DBNN]
    ret.push_str( &nickname.unwrap() );     // [LABEL]
    ret.push_str( DBNicknameRow::AFFIX );   // [::DBNN]
    ret.into_bytes()
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == DBNicknameRow::AFFIX { return true; }
    false
  }

  /// Assumes first affix has been read
  pub fn read ( f: &mut File ) -> Result<String, String> 
  {
    let mut buffer = [ 0; LABEL_BYTES ];
    let _ = f.read_exact( &mut buffer );
    let label_res = str_from_bytes( &buffer.to_vec() );
    if label_res.is_err() { return Err( String::from( "Read Error: DB Nickname" ));}
    let _ = f.seek( SeekFrom::Current(( ROW_AFFIX_BYTES ) as i64 ));
    Ok( label_res.unwrap() )
  }

  /// Assumes first affix has been read
  pub fn skip ( f: &mut File ) -> Result<u64, Error>
  {
    return f.seek( SeekFrom::Current(( LABEL_BYTES + ROW_AFFIX_BYTES ) as i64 ));
  }
}

pub struct GraphRow {}
impl GraphRow 
{
  const AFFIX: &'static str = "[::::GR]";

  pub fn new ( id: &UUID, nickname: &Label ) -> Vec<u8> 
  {
    let mut ret: String = String::from( "" );
    ret.push_str( GraphRow::AFFIX );        // [::::GR]
    ret.push_str( &id.unwrap() );           // [UUID]
    ret.push_str( &nickname.unwrap() );     // [LABEL]
    ret.push_str( GraphRow::AFFIX );        // [::::GR]
    ret.into_bytes()
  }

  pub fn from ( id: Result<UUID, String>, nickname: Result<Label, String> ) -> Result<Vec<u8>, String> 
  {
    if id.is_ok() && id.is_ok() 
    {
      return Ok( GraphRow::new( &id.unwrap(), &nickname.unwrap() ));
    }

    if id.is_err() { return Err( id.unwrap_err() ); }
    if nickname.is_err() { return Err( nickname.unwrap_err() ); }

    Err( String::from( "Error creating GraphRow" ))
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == GraphRow::AFFIX { return true; }
    false
  }

  /// Assumes affix has been read
  pub fn skip ( f: &mut File ) -> Result<u64, Error>
  {
    return f.seek( SeekFrom::Current(( UUID_BYTES + LABEL_BYTES + ROW_AFFIX_BYTES ) as i64 ));
  }

  /// Assumes affix has been read
  pub fn read ( f: &mut File ) -> Result<( String, String ), String> 
  {
    let mut uuid_buffer = [ 0; UUID_BYTES ];
    let _ = f.read_exact( &mut uuid_buffer );
    let uuid_res = str_from_bytes( &uuid_buffer.to_vec() );
    if uuid_res.is_err() { return Err( String::from( "Read Graph Row Error: UUID" ));}
    
    let mut label_buffer = [ 0; LABEL_BYTES ];
    let _ = f.read_exact( &mut label_buffer );
    let label_res = str_from_bytes( &label_buffer.to_vec() );
    if label_res.is_err() { return Err( String::from( "Read Graph Row Error: Label" ));}
    
    let _ = f.seek( SeekFrom::Current(( ROW_AFFIX_BYTES ) as i64 ));

    Ok(( uuid_res.unwrap(), label_res.unwrap() ))
  }

  pub fn size () -> usize { (ROW_AFFIX_BYTES * 2) + UUID_BYTES + LABEL_BYTES }
  pub fn cell_count () -> usize { GraphRow::size() / 8 }
}

pub struct NodeRow {}
impl NodeRow 
{
  const AFFIX: &'static str = "[::::ND]";

  pub fn new ( graph_order: &DGu64, node_id: &UUID, primary_label: &Label ) -> Vec<u8> 
  {
    let mut ret = Vec::new();
    ret.append( &mut String::from( NodeRow::AFFIX ).into_bytes() );   // [::::ND]
    ret.append( &mut graph_order.unwrap() );                          // [U64]
    ret.append( &mut node_id.unwrap().into_bytes() );                 // [UUID]
    ret.append( &mut primary_label.unwrap().into_bytes() );           // [Label]
    ret.append( &mut String::from( NodeRow::AFFIX ).into_bytes() );   // [::::ND]
    ret
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == NodeRow::AFFIX { return true; }
    false
  }
}

pub struct EdgeRow {}
impl EdgeRow 
{
  const AFFIX: &'static str = "[::::EG]";

  pub fn new (
    graph_order: &DGu64, 
    edge_id: &UUID, 
    primary_label: &Label, 
    edge_dir: &str, 
    left_uuid: &UUID, 
    right_uuid: &UUID ) -> Vec<u8> 
  {
    let mut ret = Vec::new();
    ret.append( &mut String::from( EdgeRow::AFFIX ).into_bytes() );     // [::::EG]
    ret.append( &mut graph_order.unwrap() );                            // [U64]
    ret.append( &mut edge_id.unwrap().into_bytes() );                   // [UUID]
    ret.append( &mut primary_label.unwrap().into_bytes() );             // [Label]
    ret.append( &mut edge_dir.to_string().into_bytes() );               // [EdgeDir]
    ret.append( &mut left_uuid.unwrap().into_bytes() );                 // [UUID]
    ret.append( &mut right_uuid.unwrap().into_bytes() );                // [UUID]
    ret.append( &mut String::from( EdgeRow::AFFIX ).into_bytes() );     // [::::EG]
    ret
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == EdgeRow::AFFIX { return true; }
    false
  }
}

pub struct PropRow {}
impl PropRow 
{
  //const AFFIX: &'static str = "[::::PR]";
}

#[derive( Debug, Clone, PartialEq )]
pub enum PageType { DBPage, DataPage, AJMPage }

/// Used differently than other rows
pub struct PageRow {}
impl PageRow 
{
  const DB_AFFIX: &'static str = "[::DBPG]";
  const DATA_AFFIX: &'static str = "[::DTPG]";
  const AJM_AFFIX: &'static str = "[:AJMPG]";
  const EMPTY_AFFIX: &'static str = "[:EMPTY]"; // single empty cell
  const START_EMPTY_AFFIX: &'static str = "[STEMTY]"; // start empty cells (u64s)

  pub fn new_db_affix () -> Vec<u8> { String::from( PageRow::DB_AFFIX ).into_bytes() }
  pub fn new_data_affix () -> Vec<u8> { String::from( PageRow::DATA_AFFIX ).into_bytes() }
  pub fn new_ajm_affix () -> Vec<u8> { String::from( PageRow::AJM_AFFIX ).into_bytes() }
  pub fn new_empty_affix () -> Vec<u8> { String::from( PageRow::EMPTY_AFFIX ).into_bytes() }
  pub fn new_start_empty_affix () -> Vec<u8> { String::from( PageRow::START_EMPTY_AFFIX ).into_bytes() }

  pub fn is_db_affix ( affix: &str ) -> bool 
  {
    if affix == PageRow::DB_AFFIX { return true; }
    false
  }

  pub fn is_data_affix ( affix: &str ) -> bool 
  {
    if affix == PageRow::DATA_AFFIX { return true; }
    false
  }

  pub fn is_ajm_affix ( affix: &str ) -> bool 
  {
    if affix == PageRow::AJM_AFFIX { return true; }
    false
  }

  pub fn is_empty_affix ( affix: &str ) -> bool 
  {
    if affix == PageRow::EMPTY_AFFIX { return true; }
    false
  }

  pub fn is_start_empty_affix ( affix: &str ) -> bool 
  {
    if affix == PageRow::START_EMPTY_AFFIX { return true; }
    false
  }

  pub fn empty_cell_count ( bytes: usize ) -> usize { bytes / 8 }

  pub fn gen_empty_cells ( bytes: usize ) -> Vec<u8> 
  {
    let mut ret: Vec<u8> = Vec::new();
    let mut cell_count = bytes / 8;

    if cell_count == 1 
    {
      ret.extend_from_slice( &PageRow::new_empty_affix() );
      return ret;
    }

    let mut write_start = false;
    while cell_count > 0 
    {
      if write_start == false 
      {
        ret.extend_from_slice( &PageRow::new_start_empty_affix() );
        write_start = true;
      }
      else 
      {
        let cell = ( cell_count as u64 ).to_le_bytes().to_vec();      
        ret.extend_from_slice( &cell );
      }
      cell_count -= 1;
    }
    ret
  }

  pub fn read_next_empty ( f: &mut File ) -> Result<u64, String> { Ok( next_u64(f) )}

  /// Assumes start empty affix has been read
  pub fn skip_empty_cells ( f: &mut File ) -> Result<(u64, u64), Error>
  {
    let res = PageRow::read_next_empty( f );
    if res.is_ok() 
    {
      let seek_res = f.seek( SeekFrom::Current(((( res.as_ref().unwrap() - 1 ) * 8 )) as i64 ));
      if seek_res.is_ok() { return Ok(( seek_res.unwrap(), res.unwrap() )); }
    }
    Err( Error::new(std::io::ErrorKind::Other, String::from( "Error skipping empty cells" )))
  }

  // data page with no data
  pub fn data_page_size () -> usize { (ROW_AFFIX_BYTES * 3) + GraphRow::size() }
}

#[derive( Debug, Clone, PartialEq )]
pub enum AffixType 
{ 
  DBPage, DataPage, AJMPage,
  BuildId, DBNickname,
  Graph, Empty, StartEmpty,
  Placeholder, End
}

pub fn affix_to_type ( affix: &str ) -> Option<AffixType> 
{
  if PageRow::is_db_affix( affix ) { return Some( AffixType::DBPage ); }
  if PageRow::is_data_affix( affix ) { return Some( AffixType::DataPage ); }
  if PageRow::is_ajm_affix( affix ) { return Some( AffixType::AJMPage ); }
  if PageRow::is_empty_affix( affix ) { return Some( AffixType::Empty ); }
  if PageRow::is_start_empty_affix( affix ) { return Some( AffixType::StartEmpty ); }
  if BuildIDRow::is_affix( affix ) { return Some( AffixType::BuildId ); }
  if DBNicknameRow::is_affix( affix ) { return Some( AffixType::DBNickname ); }
  if GraphRow::is_affix( affix ) { return Some( AffixType::Graph ); }
  if is_end_affix( affix ) { return Some( AffixType::End ); }
  if is_placeholder_affix( affix ) { return Some( AffixType::Placeholder ); }
  None
}

pub fn is_end_affix ( affix: &str ) -> bool 
{
  if affix == END_DB { return true; }
  false
}

pub fn is_placeholder_affix ( affix: &str ) -> bool 
{
  if affix == PLACEHOLDER { return true; }
  false
}

pub fn is_default_graph ( name: &str ) -> bool
{
  if name == "DEFAULT_GRAPH\\::::::::::::::::::::::::::::::::::::::::::::::::::" { return true; }
  false
}

#[cfg( test )]
mod tests 
{
  use super::*;
  //use utils::{ pad_str };

  #[test]
  fn test_cons_graph_row () 
  {
    //let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let nickname = pad_str( LABEL_BYTES, String::from( "nickname" ));
    //let row = GraphRow::new( &graph_id, &nickname );
    //assert_eq!( row.len(), 120 );
  }

  #[test]
  fn test_build_id_row () 
  {
    let uuid_res = UUID::new( String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ));
    let row = BuildIDRow::new( &uuid_res.unwrap() );
    assert_eq!( row.len(), 56 );
  }

  #[test]
  fn test_cons_node_row () 
  {
    //let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let node_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let primary_label = pad_str( LABEL_BYTES, String::from( "node1" ));
    //let row: Vec<u8> = NodeRow::new( &graph_id, &node_id, &primary_label, false );
    //assert_eq!( row.len(), 168 );
  }

  #[test]
  fn test_cons_edge_row () 
  {
    //let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let edge_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let primary_label = pad_str( LABEL_BYTES, String::from( "edge1" ));
    //let edge_dir = pad_str( LABEL_BYTES, String::from( "edge dir" ));
    //let left_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    //let right_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );

    //let row = EdgeRow::new( &graph_id, &edge_id, &primary_label, &edge_dir, &left_uuid, &right_uuid, false );
    //assert_eq!( row.len(), 312 );
  }
}