use std::io::{ Read };
use std::fs::{ File };
use utils::{ str_from_bytes };
use crate::enums::{ DGUUID_BYTES, DGLABEL_BYTES, ROW_PREFIX_BYTES };

// len is number of bytes (not characters)
pub fn validate_dg_uuid ( uuid: &str ) -> bool { uuid.len() == DGUUID_BYTES }

pub fn validate_dg_label ( label: &str ) -> bool { label.len() == DGLABEL_BYTES }





#[cfg(test)]
mod tests 
{
  use super::*;
  use std::path::PathBuf;
  use std::io::{ Seek, SeekFrom };
  use utils::{ pad_str, open_file };
  use crate::enums::{ SDBCONFIG_PAGE_START_BYTES, ROW_PREFIX_GRAPH_REF };

  #[test]
  fn test_validate_dg_uuid () 
  {
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ), true );
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c88" ), false );
    assert_eq!( validate_dg_uuid( "67e55044-10b1-426f-9247-bb680e5fe0c" ), false );
    assert_eq!( validate_dg_uuid( "" ), false );
  }

  #[test]
  fn test_validate_dg_label () 
  {
    assert_eq!( validate_dg_label( &pad_str( DGLABEL_BYTES, String::from( "test" ))), true );
    assert_eq!( validate_dg_label( "test" ), false );
    assert_eq!( validate_dg_label( "" ), false );
    assert_eq!( validate_dg_label( "----------------------------------------------------------------------" ), false );
  }

  #[test]
  fn test_next_row_prefix () 
  {
    let path = PathBuf::from( "./test_data/sf_db_1_ref.sdb" );
    let open_res = open_file( &path );
    let mut f = open_res.unwrap();

    let _seek_res = f.seek( SeekFrom::Start( SDBCONFIG_PAGE_START_BYTES as u64 ));
    let prefix_opt = next_row_prefix( &mut f );
    assert_eq!( prefix_opt.is_some(), true );
    assert_eq!( prefix_opt.unwrap(), ROW_PREFIX_GRAPH_REF );
  }

  #[test]
  fn test_next_label () 
  {
    let path = PathBuf::from( "./test_data/sf_db_1_ref.sdb" );
    let open_res = open_file( &path );
    let mut f = open_res.unwrap();

    let _seek_res = f.seek( SeekFrom::Start( SDBCONFIG_PAGE_START_BYTES as u64 ));
    let prefix_opt = next_row_prefix( &mut f );
    assert_eq!( prefix_opt.is_some(), true );
    assert_eq!( prefix_opt.unwrap(), ROW_PREFIX_GRAPH_REF );

    let label_opt = next_label( &mut f );
    assert_eq!( label_opt.is_some(), true );
    assert_eq!( label_opt.unwrap(), String::from( "graph1\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));
  }
}