#![allow( dead_code )]
pub mod enums;
pub mod structs;
pub mod dg_utils;
pub mod pages;
pub mod rows;

//use std::string::{ FromUtf8Error };


/* !!
  - all inputs will get translated into a transaction
  - all writes need to end with a write complete bit [WC] to ensure a good write.
  - files also need to end with a TransactionComplete UUID to ensure good transaction write.
  - If either the wc bit or the transactionComplete bit are missing, the db is in a bad state.
*/




// ---------------------------------------------------------------------------------------------------------------------


// ---------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests 
{
  use super::*;
  use std::path::PathBuf;
  use std::fs::{ OpenOptions, File };
  use std::io::{ BufWriter, Error, Read, Write };
  use utils::{ pad_str, str_to_bytes, str_from_bytes };

  #[test]
  fn test_string_binary () 
  {
    let bytes: Vec<_> = b"test".to_vec();
    assert_eq!( bytes, [116, 101, 115, 116] );

    let string = String::from_utf8( bytes ).expect( "invalid utf8" );
    assert_eq!( string, "test" );

    let bytes: Vec<_> = b"/".to_vec();
    assert_eq!( bytes.len(), 1 );

    let bytes: Vec<_> = b"[".to_vec();
    assert_eq!( bytes.len(), 1 );

    let bytes: Vec<_> = b"]".to_vec();
    assert_eq!( bytes.len(), 1 );
  }

  #[test]
  fn test_vec_slices () 
  {
    let bytes: Vec<_> = str_to_bytes( String::from( "testtest" ) ).to_vec();
    assert_eq!( &bytes[0..4], [116, 101, 115, 116] );
    assert_eq!( &bytes[4..8], [116, 101, 115, 116] );
    
    let res1 = str_from_bytes( &bytes[0..4] );
    assert_eq!( res1.is_ok(), true );
    assert_eq!( res1.unwrap(), "test" );

    let res2 = str_from_bytes( &bytes[4..8] );
    assert_eq!( res2.is_ok(), true );
    assert_eq!( res2.unwrap(), "test" );
  }

  #[test]
  fn test_str_len () 
  {
    let astr = String::from( "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" );
    assert_eq!( astr.into_bytes().len(), 64 );
  
    let esc = String::from( "\\" );
    assert_eq!( esc.into_bytes().len(), 1 );
  }  

  #[test]
  fn test_u64_binary () 
  {
    assert_eq!( 0x123u64.to_le_bytes().len(), 8 );
    assert_eq!( 0x1234567890123456u64.to_le_bytes().len(), 8 );
    assert_eq!( 0x0u64.to_le_bytes().len(), 8 );
  }

  #[test]
  fn test_uuid () 
  {
    let double_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" );
    assert_eq!( double_uuid.into_bytes().len(), 36 );
  }

  #[test]
  fn test_str_length () 
  {
    let astr = String::from( "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" );
    assert_eq!( astr.into_bytes().len(), 64 );
  }

  

  #[test]
  fn test_common_none () 
  {
    assert_eq!( COMMON_NONE.bytes().len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_dgsdbconfig () 
  {
    /*
    let dg_config_1_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_1_res.is_err(), true );
    assert_eq!( dg_config_1_res, Err( DataGramError::InvalidDGUUID ));

    let dg_config_2_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "----------------------------------------------------------------------" )));

    assert_eq!( dg_config_2_res.is_err(), true );
    assert_eq!( dg_config_2_res, Err( DataGramError::InvalidDGCommonString ));
    
    let dg_config_3_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_3_res.is_ok(), true );

    let dg_config_3 = dg_config_3_res.unwrap();
    assert_eq!( dg_config_3.build_id.len(), ByteLengths::UUID as usize );
    assert_eq!( dg_config_3.nickname.len(), ByteLengths::CommonString as usize );
    */
  }

  #[test]
  fn test_datagram_writer () 
  {
    //let path = PathBuf::from( "/platonic3d/subgraphdbv2/test_data/sf/unit_tests/test_datagram_writer.sdb" );
    //let file_res: Result<File, Error> = OpenOptions::new()
    //  .read( true )
    //  .write( true )
    //  .truncate( true )
    //  .create( true )
    //  .open( path );
    //assert_eq!( COMMON_NONE.bytes().len(), ByteLengths::CommonString as usize );
  }

  #[test]
  fn test_dgsdbconfig_to_bytes () 
  {
    /*
    let dg_config_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));

    assert_eq!( dg_config_res.is_ok(), true );
    assert_eq!( dg_config_res.unwrap().to_bytes().len(), 100 );
    */
  }

  #[test]
  fn test_dgsdbconfig_from_bytes () 
  {
    /*
    // --- some
    let dg_config_res = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      Some( String::from( "nickname" )));
    assert_eq!( dg_config_res.is_ok(), true );

    let bytes = dg_config_res.unwrap().to_bytes();
    assert_eq!( bytes.len(), 100 );
    
    let res = DGSDBConfig::from_bytes( bytes );   
    assert_eq!( res.is_ok(), true );
    
    let config = res.unwrap();
    assert_eq!( config.build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    assert_eq!( config.nickname, pad_str( ByteLengths::CommonString as usize, String::from( "nickname" )));
    
    // -- none
    let dg_config_res1 = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      None );
    assert_eq!( dg_config_res1.is_ok(), true );

    let bytes1 = dg_config_res1.unwrap().to_bytes();
    assert_eq!( bytes1.len(), 100 );

    let res1 = DGSDBConfig::from_bytes( bytes1 );   
    assert_eq!( res1.is_ok(), true );
    
    let config1 = res1.unwrap();
    assert_eq!( config1.build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    assert_eq!( config1.nickname, COMMON_NONE );


    // -- error
    let dg_config_res2 = DGSDBConfig::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      None );
    assert_eq!( dg_config_res2.is_ok(), true );

    let bytes2 = dg_config_res2.unwrap().to_bytes();
    assert_eq!( bytes2.len(), 100 );

    let res2 = DGSDBConfig::from_bytes( bytes2[0..99].to_vec() );   
    assert_eq!( res2.is_err(), true );
    */
  }
}