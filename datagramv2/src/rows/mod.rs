use std::io::{ Seek, SeekFrom, Read, Error };
use std::fs::{ File };
use utils::{ str_from_bytes };
use common::{ UUID_BYTES, LABEL_BYTES, ROW_AFFIX_BYTES, bool_to_affix };

pub static NODE_ROW_AFFIX: &'static str = "";
pub static EDGE_ROW_AFFIX: &'static str = "[::::EG]";

// !! ALL VALUES MUST BE PADDED !!

pub struct BuildIDRow {}
impl BuildIDRow 
{
  // [::::BI][UUID][::::BI]
  const AFFIX: &'static str = "[::::BI]";

  pub fn new ( build_id: &str ) -> Vec<u8> 
  {
    let mut ret: String = String::from( BuildIDRow::AFFIX );
    ret.push_str( build_id );
    ret.push_str( BuildIDRow::AFFIX );
    ret.into_bytes()
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == BuildIDRow::AFFIX { return true; }
    false
  }

  /// Assumes affix has been read
  pub fn skip ( f: &mut File ) -> Result<u64, Error>
  {
    return f.seek( SeekFrom::Current(( UUID_BYTES + ROW_AFFIX_BYTES ) as i64 ));
  }
}

pub struct DBNicknameRow {}
impl DBNicknameRow 
{
  // [::DBNN][LABEL][::DBNN]
  const AFFIX: &'static str = "[::DBNN]";

  pub fn new ( nickname: &str ) -> Vec<u8> 
  {
    let mut ret: String = String::from( DBNicknameRow::AFFIX);
    ret.push_str( nickname );
    ret.push_str( DBNicknameRow::AFFIX );
    ret.into_bytes()
  }

  pub fn is_affix ( affix: &str ) -> bool 
  {
    if affix == DBNicknameRow::AFFIX { return true; }
    false
  }

  /// Assumes affix has been read
  pub fn skip ( f: &mut File ) -> Result<u64, Error>
  {
    return f.seek( SeekFrom::Current(( LABEL_BYTES + ROW_AFFIX_BYTES ) as i64 ));
  }
}

pub struct GraphRow {}
impl GraphRow 
{
  // [::::GR][UUID][LABEL][::::GR]
  const AFFIX: &'static str = "[::::GR]";

  pub fn new ( id: &str, nickname: &str ) -> Vec<u8> 
  {
    let mut ret: String = String::from( GraphRow::AFFIX );
    ret.push_str( id );
    ret.push_str( nickname );
    ret.push_str( GraphRow::AFFIX );
    ret.into_bytes()
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
    
    Ok(( uuid_res.unwrap(), label_res.unwrap() ))
  }
}

pub struct NodeRow {}
impl NodeRow 
{
  // AFFIX UUID UUID LABEL BOOL AFFIX 
  const AFFIX: &'static str = "[::::ND]";

  pub fn new ( graph_id: &str, node_id: &str, primary_label: &str, has_props: bool ) -> Vec<u8> 
  {
    let mut ret: String = String::from( NodeRow::AFFIX );
    ret.push_str( graph_id );
    ret.push_str( node_id );
    ret.push_str( primary_label );
    ret.push_str( bool_to_affix( has_props ));
    ret.push_str( NodeRow::AFFIX );
    ret.into_bytes()
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
  // AFFIX UUID UUID LABEL LABEL LABEL LABEL BOOL AFFIX 
  const AFFIX: &'static str = "[::::EG]";

  pub fn new (
    graph_id: &str, edge_id: &str, primary_label: &str, edge_dir: &str, left_uuid: &str, 
    right_uuid: &str, has_props: bool ) -> Vec<u8> 
  {
    let mut ret: String = String::from( EdgeRow::AFFIX );
    ret.push_str( graph_id );
    ret.push_str( edge_id );
    ret.push_str( primary_label );
    ret.push_str( edge_dir );
    ret.push_str( left_uuid );
    ret.push_str( right_uuid );
    ret.push_str( bool_to_affix( has_props ));
    ret.push_str( EdgeRow::AFFIX );
    ret.into_bytes()
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
  const AFFIX: &'static str = "[::::PR]";
}



#[cfg( test )]
mod tests 
{
  use super::*;
  use utils::{ pad_str };

  #[test]
  fn test_cons_graph_row () 
  {
    let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let nickname = pad_str( LABEL_BYTES, String::from( "nickname" ));
    let row = GraphRow::new( &graph_id, &nickname );
    assert_eq!( row.len(), 120 );
  }

  #[test]
  fn test_build_id_row () 
  {
    let build_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let row = BuildIDRow::new( &build_id );
    assert_eq!( row.len(), 56 );
  }

  #[test]
  fn test_cons_node_row () 
  {
    let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let node_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let primary_label = pad_str( LABEL_BYTES, String::from( "node1" ));
    let row: Vec<u8> = NodeRow::new( &graph_id, &node_id, &primary_label, false );
    assert_eq!( row.len(), 168 );
  }

  #[test]
  fn test_cons_edge_row () 
  {
    let graph_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let edge_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let primary_label = pad_str( LABEL_BYTES, String::from( "edge1" ));
    let edge_dir = pad_str( LABEL_BYTES, String::from( "edge dir" ));
    let left_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let right_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );

    let row = EdgeRow::new( &graph_id, &edge_id, &primary_label, &edge_dir, &left_uuid, &right_uuid, false );
    assert_eq!( row.len(), 312 );
  }
}