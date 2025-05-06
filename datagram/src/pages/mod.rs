use std::io::{ Seek, SeekFrom, Read };
use std::fs::{ File };
use utils::{ pad_str, str_from_bytes, parse_padded_str };
use crate::enums::{ 
  DGLABEL_BYTES, 
  SDBCONFIG_PAGE_START_BYTES, 
  SDBCONFIG_PAGE_END_BYTES, 
  DataGramError, 
  EMPTY_BYTE,
  SDBCONFIG_GRAPH_REF_BYTES,
  ROW_PREFIX_GRAPH_REF
};
use crate::structs::{ GraphRef };
use crate::dg_utils::{ validate_dg_uuid, validate_dg_label, next_row_prefix, next_label };
use crate::rows::{ sdb_config_page_start, sdb_config_page_end, sdb_config_graph_ref, empty_row };

// SDB Config Page (Byte Interface)
#[derive( Debug, PartialEq )]
pub struct SDBConfigPage
{
  build_id: String, // DGUUID
  nickname: String, // DGLabel
  graph_refs: Vec<GraphRef>,
}

impl SDBConfigPage
{
  pub fn new ( build_id: String, nickname: String, graph_refs: Vec<GraphRef> ) -> Result<SDBConfigPage, DataGramError> 
  {
    if !validate_dg_uuid( &build_id ) { return Err( DataGramError::InvalidDGUUID )}
    if nickname.len() == 0 { return Err( DataGramError::InvalidDGLabel )}

    let nickname_actual = pad_str( DGLABEL_BYTES, nickname );
    if !validate_dg_label( &nickname_actual ) { return Err( DataGramError::InvalidDGLabel )}
    return Ok( SDBConfigPage { build_id: build_id, nickname: nickname_actual, graph_refs: graph_refs })
  }

  pub fn to_rows ( &self, page_size: &usize ) -> Vec<u8>
  {
    let mut ret:Vec<u8> = Vec::new();
    ret.append( &mut sdb_config_page_start( &self.build_id, &self.nickname ));
    
    for gr in &self.graph_refs { ret.append( &mut sdb_config_graph_ref( gr ));}

    let remaining_bytes = page_size - ( ret.len() + SDBCONFIG_PAGE_END_BYTES );
    ret.append( &mut empty_row( remaining_bytes / 8 ));
    
    ret.append( &mut sdb_config_page_end( &self.build_id, &self.nickname ));
    ret
  }

  pub fn has_empty_bytes ( page_size: &usize, f: &mut File ) -> Result<u64, DataGramError>
  {
    let _seek_res = f.seek( SeekFrom::Start( SDBConfigPage::seek_position_begin_graph_refs() as u64 ));
    let max_rows = SDBConfigPage::max_rows( page_size );
    let mut n = 0;    
    while n < max_rows 
    {
      let mut buffer = [ 0; 8 ];
      let _ = f.read_exact( &mut buffer );
      let cast_res = str_from_bytes( &buffer.to_vec() );
      if cast_res.is_ok() 
      {
        let byte = cast_res.unwrap();
        if byte == EMPTY_BYTE { return Ok( f.stream_position().unwrap() ); }
        if byte == ROW_PREFIX_GRAPH_REF 
        { 
          let _ = f.seek( SeekFrom::Current(( SDBCONFIG_GRAPH_REF_BYTES - 8 ).try_into().unwrap() ));
        }
      }
      n += 1;
    }

    Ok( 0 )
  }

  pub fn seek_position_begin_graph_refs () -> usize { SDBCONFIG_PAGE_START_BYTES }
  
  pub fn max_rows ( page_size: &usize ) -> usize 
  { 
    ( page_size - SDBCONFIG_PAGE_START_BYTES - SDBCONFIG_PAGE_END_BYTES ) / SDBCONFIG_GRAPH_REF_BYTES
  }

  // !!! This needs to return an error
  pub fn graph_exists ( graph_name: &str, page_size: &usize, f: &mut File ) -> Result<bool, DataGramError> 
  {
    let _seek_res = f.seek( SeekFrom::Start( SDBConfigPage::seek_position_begin_graph_refs() as u64 ));
    let max_rows = SDBConfigPage::max_rows( page_size );
    let mut n = 0;    
    while n < max_rows 
    {
      let prefix_opt = next_row_prefix( f );
      if prefix_opt.is_some() 
      {
        let prefix_byte = prefix_opt.unwrap();
        if prefix_byte == EMPTY_BYTE { return Ok( false ); }
        if prefix_byte == ROW_PREFIX_GRAPH_REF 
        { 
          let nickname_opt = next_label( f ); // grab graph nickname
          if nickname_opt.is_some() 
          {
            if graph_name == parse_padded_str( &nickname_opt.unwrap() ) { return Ok( true ); }
          }
        }
      }
      n += 1;
    }
    Ok( false )
  }
}

pub struct GraphNodesPage {}
pub struct GraphEdgesPage {}
pub struct NEPropsPage {}
pub struct EmptyPage {} // for deletions
pub struct ChangeLogPage {}
pub struct IndexPage {}
pub struct MatrixPage {}

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::path::PathBuf;
  use utils::{ open_file };

  #[test]
  fn test_sdb_config_page_new () 
  {
    let config: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "" ), 
      String::from( "" ),
      Vec::new() );
    assert_eq!( config.is_err(), true );
    assert_eq!( config, Err( DataGramError::InvalidDGUUID ));

    // ---
    let config1: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "" ),
      Vec::new() );
    assert_eq!( config1.is_err(), true );
    assert_eq!( config1, Err( DataGramError::InvalidDGLabel ));

    // ---
    let config2: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );
    assert_eq!( config2.is_ok(), true );
    assert_eq!( config2.as_ref().unwrap().build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ));
    assert_eq!( config2.as_ref().unwrap().nickname, 
      pad_str( DGLABEL_BYTES, String::from( "nickname" )));

    // ---
    let gr_res = GraphRef::new( String::from( "graph1" ));
    let mut grs = Vec::new();
    grs.push( gr_res.unwrap() );
    let config3: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      grs );
    assert_eq!( config3.is_ok(), true );
    assert_eq!( config3.as_ref().unwrap().graph_refs.len(), 1 );
  }

  #[test]
  fn test_sdb_config_page_to_rows () 
  {
    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      Vec::new() );
    assert_eq!( config_res.is_ok(), true );
    assert_eq!( config_res.unwrap().to_rows( &4096 ).len(), 4096 );

    // ---
    let gr_res = GraphRef::new( String::from( "graph1" ));
    let mut grs = Vec::new();
    grs.push( gr_res.unwrap() );
    let config2: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), 
      String::from( "nickname" ),
      grs );
    assert_eq!( config2.is_ok(), true );
    assert_eq!( config2.unwrap().to_rows( &4096 ).len(), 4096 );
  }

  #[test]
  fn test_seek_position_begin_graph_refs () 
  {
    assert_eq!( SDBConfigPage::seek_position_begin_graph_refs(), 128 as usize );
  }

  #[test]
  fn test_max_rows () 
  {
    assert_eq!( SDBConfigPage::max_rows( &4096 ), 48 as usize );
  }

  #[test]
  fn test_sdb_config_page_has_empty_bytes () 
  {
    let page_size: usize = 4096;
    
    // ---
    let path = PathBuf::from( "./test_data/new_sf_db.sdb" );
    let open_res = open_file( &path );
    let mut file = open_res.unwrap();
    let res = SDBConfigPage::has_empty_bytes( &page_size, &mut file );

    assert_eq!( res.is_ok(), true );
    assert_eq!( res.unwrap(), 136 );

    // ---
    let path1 = PathBuf::from( "./test_data/sf_db_1_ref.sdb" );
    let open_res1 = open_file( &path1 );
    let mut file1 = open_res1.unwrap();
    let res1 = SDBConfigPage::has_empty_bytes( &page_size, &mut file1 );

    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), 216 );
  }

  #[test]
  fn test_graph_exists () 
  {
    let page_size: usize = 4096;
    let path = PathBuf::from( "./test_data/sf_db_1_ref.sdb" );
    let path10 = PathBuf::from( "./test_data/sf_db_10_ref.sdb" );

    // ---
    let open_res = open_file( &path );
    let mut file = open_res.unwrap();
    let res = SDBConfigPage::graph_exists( "test", &page_size, &mut file );

    assert_eq!( res.is_ok(), true );
    assert_eq!( res.unwrap(), false );

    // ---
    let open_res1 = open_file( &path );
    let mut file1 = open_res1.unwrap();
    let res1 = SDBConfigPage::graph_exists( "graph1", &page_size, &mut file1 );

    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), true );

    // ---
    let open_res1 = open_file( &path10 );
    let mut file1 = open_res1.unwrap();
    let res1 = SDBConfigPage::graph_exists( "graph1", &page_size, &mut file1 );

    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), true );

    // ---
    let open_res1 = open_file( &path10 );
    let mut file1 = open_res1.unwrap();
    let res1 = SDBConfigPage::graph_exists( "graph10", &page_size, &mut file1 );

    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), true );

    // ---
    let open_res1 = open_file( &path10 );
    let mut file1 = open_res1.unwrap();
    let res1 = SDBConfigPage::graph_exists( "graph11", &page_size, &mut file1 );

    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), false );
  }
}