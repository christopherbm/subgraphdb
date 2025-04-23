use crate::structs::{ GraphRef };

// These all Need to Be 8 Bites
pub static WRITE_COMPLETE: &'static str = "[::::WC]";
pub static PAGE_START: &'static str = "PGE::STR";
pub static PAGE_END: &'static str = "PGE::END";
pub static SDB_CONFIG_PAGE: &'static str = "[::SDBC]"; // page type (4 bytes)
pub static ROW_PREFIX_GRAPH_INFO: &'static str = "[::::GI]";

// ---------------------------------------------------------------------------------------------------------------------
// [PAGESTART build_id nickname WC]
pub fn sdb_config_page_start ( build_id: &str, padded_nickname: &str ) -> Vec<u8>
{
  let mut ret: String = String::from( PAGE_START );
  ret.push_str( SDB_CONFIG_PAGE );
  ret.push_str( build_id );
  ret.push_str( padded_nickname );
  ret.push_str( WRITE_COMPLETE );
  ret.into_bytes()
}

// [PAGEEND nickname WC]
pub fn sdb_config_page_end ( padded_nickname: &str ) -> Vec<u8>
{
  let mut ret: String = String::from( PAGE_END );
  ret.push_str( SDB_CONFIG_PAGE );
  ret.push_str( padded_nickname );
  ret.push_str( WRITE_COMPLETE );
  ret.into_bytes()
}

// [GI nickname order WC]
pub fn sdb_config_graph_ref ( graph_ref: &GraphRef ) -> Vec<u8> 
{
  let mut ret: String = String::from( ROW_PREFIX_GRAPH_INFO ); 
  ret.push_str( &graph_ref.nickname );

  let mut ret_vec:Vec<u8> = Vec::new();
  ret_vec.append( &mut ret.into_bytes() );
  ret_vec.append( &mut Vec::from( graph_ref.order.to_le_bytes() ));
  ret_vec.append( &mut Vec::from( WRITE_COMPLETE.as_bytes() ));
  ret_vec
}
// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests 
{
  use super::*;
  use utils::{ pad_str };
  use crate::enums::{ ByteLengths, DGRowLength };

  #[test]
  fn test_sdb_config_page_start () 
  {
    let build_id = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" );
    let nickname = pad_str( ByteLengths::CommonString as usize, String::from( "nickname" ));
    assert_eq!( sdb_config_page_start( &build_id, &nickname ).len(), 124 );
    assert_eq!( sdb_config_page_start( &build_id, &nickname ).len(), DGRowLength::SDBConfigPageStart as usize );

    // --- 
    let mut comp: String = String::from( "PAGESTARTSDBC" );
    comp.push_str( &build_id );
    comp.push_str( &nickname );
    comp.push_str( WRITE_COMPLETE );
    //println!( "{:?}", comp );
    assert_eq!( sdb_config_page_start( &build_id, &nickname ), comp.into_bytes() );
  }

  #[test]
  fn test_sdb_config_page_end () 
  {
    let nickname = pad_str( ByteLengths::CommonString as usize, String::from( "nickname" ));
    assert_eq!( sdb_config_page_end( &nickname ).len(), 88 );
    assert_eq!( sdb_config_page_end( &nickname ).len(), DGRowLength::SDBConfigPageEnd as usize );

    // --- 
    let mut comp: String = String::from( "PAGEENDSDBC" );
    comp.push_str( &nickname );
    comp.push_str( WRITE_COMPLETE );
    assert_eq!( sdb_config_page_end( &nickname ), comp.into_bytes() );
  }

  #[test]
  fn test_sdb_config_graph_ref () 
  {
    let graph_res_1 = GraphRef::new( String::from( "graph" ), 5 );
    assert_eq!( sdb_config_graph_ref( graph_res_1.as_ref().unwrap() ).len(), 88 );
    assert_eq!( 
      sdb_config_graph_ref( 
        graph_res_1.as_ref().unwrap() ).len(), DGRowLength::SDBConfigPageEnd as usize );
  }

  #[test]
  fn test_static_strs () 
  {
    assert_eq!( WRITE_COMPLETE.bytes().len(), 8 );
    assert_eq!( PAGE_START.bytes().len(), 8 );
    assert_eq!( PAGE_END.bytes().len(), 8 );
    assert_eq!( SDB_CONFIG_PAGE.bytes().len(), 8 );
    assert_eq!( ROW_PREFIX_GRAPH_INFO.bytes().len(), 8 );
  }
}