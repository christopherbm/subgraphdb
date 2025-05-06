use crate::structs::{ GraphRef };
use crate::enums::{ PAGE_START, SDB_CONFIG_PAGE, PAGE_END, WRITE_COMPLETE, ROW_PREFIX_GRAPH_REF, EMPTY_BYTE };

// ---------------------------------------------------------------------------------------------------------------------
pub fn sdb_config_page_start ( build_id: &str, padded_nickname: &str ) -> Vec<u8>
{
  let mut ret: String = String::from( PAGE_START );
  ret.push_str( SDB_CONFIG_PAGE );
  ret.push_str( build_id );
  ret.push_str( padded_nickname );
  ret.push_str( WRITE_COMPLETE );
  ret.into_bytes()
}


pub fn sdb_config_page_end ( build_id: &str, padded_nickname: &str ) -> Vec<u8>
{
  let mut ret: String = String::from( PAGE_END );
  ret.push_str( SDB_CONFIG_PAGE );
  ret.push_str( build_id );
  ret.push_str( padded_nickname );
  ret.push_str( WRITE_COMPLETE );
  ret.into_bytes()
}


pub fn sdb_config_graph_ref ( graph_ref: &GraphRef ) -> Vec<u8> 
{
  let mut ret: String = String::from( ROW_PREFIX_GRAPH_REF ); 
  ret.push_str( &graph_ref.nickname );
  ret.push_str( WRITE_COMPLETE );
  ret.into_bytes()
}

pub fn graph_nodes_page_start () {}
pub fn graph_nodes_page_end () {}
// uuid primary label
pub fn node_primary () {}

pub fn graph_edges_page_start () {}
pub fn graph_edges_page_end () {}

// uuid primary label
pub fn edge_primary () {}

pub fn _start () {}
pub fn ne_props_page_start () {}
pub fn ne_props_page_end () {}

pub fn empty_row ( count: usize ) -> Vec<u8> 
{
  let mut ret: String = String::from( "" );
  let mut n = 0;
  while n < count 
  {
    ret.push_str( EMPTY_BYTE );
    n += 1;
  }
  ret.into_bytes()
}
// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests 
{
  use super::*;
  use utils::{ pad_str };
  use crate::enums::{ DGLABEL_BYTES, SDBCONFIG_PAGE_START_BYTES, SDBCONFIG_PAGE_END_BYTES, SDBCONFIG_GRAPH_REF_BYTES };

  #[test]
  fn test_sdb_config_page_start () 
  {
    let build_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let nickname = pad_str( DGLABEL_BYTES, String::from( "nickname" ));
    assert_eq!( sdb_config_page_start( &build_id, &nickname ).len(), 128 );
    assert_eq!( sdb_config_page_start( &build_id, &nickname ).len(), SDBCONFIG_PAGE_START_BYTES );
  }

  #[test]
  fn test_sdb_config_page_end () 
  {
    let build_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" );
    let nickname = pad_str( DGLABEL_BYTES, String::from( "nickname" ));
    assert_eq!( sdb_config_page_end( &build_id, &nickname ).len(), 128 );
    assert_eq!( sdb_config_page_end( &build_id, &nickname ).len(), SDBCONFIG_PAGE_END_BYTES );
  }

  #[test]
  fn test_sdb_config_graph_ref () 
  {
    let graph_res_1 = GraphRef::new( String::from( "graph" ));
    assert_eq!( sdb_config_graph_ref( graph_res_1.as_ref().unwrap() ).len(), 80 );
    assert_eq!( 
      sdb_config_graph_ref( 
        graph_res_1.as_ref().unwrap() ).len(), SDBCONFIG_GRAPH_REF_BYTES );
  }

  #[test]
  fn test_empty_row () 
  {
    assert_eq!( empty_row( 0 ).len(), 0 );
    assert_eq!( empty_row( 1 ).len(), 8 );
    assert_eq!( empty_row( 2 ).len(), 16 );
  }

  #[test]
  fn test_static_strs () 
  {
    assert_eq!( WRITE_COMPLETE.bytes().len(), 8 );
    assert_eq!( PAGE_START.bytes().len(), 8 );
    assert_eq!( PAGE_END.bytes().len(), 8 );
    assert_eq!( SDB_CONFIG_PAGE.bytes().len(), 8 );
    assert_eq!( ROW_PREFIX_GRAPH_REF.bytes().len(), 8 );
    assert_eq!( EMPTY_BYTE.bytes().len(), 8 );
  }
}