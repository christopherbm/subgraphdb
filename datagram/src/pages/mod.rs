#![allow(non_snake_case)]
use utils::{ pad_str };
use crate::enums::{ DGLabelBytes, DataGramError };
use crate::structs::{ GraphRef };
use crate::dg_utils::{ validate_dg_uuid, validate_dg_label };
use crate::rows::{ sdb_config_page_start, sdb_config_page_end, sdb_config_graph_ref };

// SDB Config Page (Byte Interface)
#[derive( Debug, PartialEq )]
pub struct SDBConfigPage
{
  build_id: String, // DGUUID
  nickname: String, // DGLabel
  graph_refs: Vec<GraphRef>,
}

/*
pub fn to_bytes ( &self ) -> Vec<u8> { [ self.build_id.as_bytes(), self.nickname.as_bytes() ].concat() }

pub fn from_bytes ( bytes: Vec<u8> ) -> Result<DGSDBConfig, DataGramError> 
{
  if bytes.len() != 100 { return Err( DataGramError::ErrorReadingSDBConfig ) }
  let build_id_res = str_from_bytes( &bytes[0..( ByteLengths::UUID as usize )]);
  let nickname_res = str_from_bytes( &bytes[
    ( ByteLengths::UUID as usize )
    ..
    ( ByteLengths::UUID as usize + ByteLengths::CommonString as usize )]);

  if build_id_res.is_ok() && nickname_res.is_ok() 
  {
    if nickname_res.as_ref().unwrap() == &COMMON_NONE 
    {
      return DGSDBConfig::new( build_id_res.unwrap(), None )
    }
    return DGSDBConfig::new( build_id_res.unwrap(), Some( nickname_res.unwrap() ))
  }
  Err( DataGramError::ErrorReadingSDBConfig )
}

/// Write DGSDBConfig
pub fn write_sdb_config ( file: &mut File, build_id: String, nickname: Option<String> ) -> Result<File, FDMError> 
{
  let sdb_config_res = DGSDBConfig::new( build_id, nickname );
  if sdb_config_res.is_ok() 
  {
    let mut stream = BufWriter::new( file );
    stream.write( &sdb_config_res.unwrap().to_bytes() ).unwrap();
    stream.flush().unwrap();
  }
  Err( FDMError::ErrorWritingSDBConfig )
}

/// Read DGSDBConfig
pub fn read_sdb_config ( mut file: &File ) -> Result<File, FDMError> 
{
  Err( FDMError::ErrorReadingSDBConfig )
}
*/

impl SDBConfigPage
{
  pub fn new ( build_id: String, nickname: String, graph_refs: Vec<GraphRef> ) -> Result<SDBConfigPage, DataGramError> 
  {
    if !validate_dg_uuid( &build_id ) { return Err( DataGramError::InvalidDGUUID )}
    if nickname.len() == 0 { return Err( DataGramError::InvalidDGLabel )}

    let nickname_actual = pad_str( DGLabelBytes, nickname );
    if !validate_dg_label( &nickname_actual ) { return Err( DataGramError::InvalidDGLabel )}
    return Ok( SDBConfigPage { build_id: build_id, nickname: nickname_actual, graph_refs: graph_refs })
  }

  pub fn to_rows ( &self, page_size: &u32 ) -> Vec<u8>
  {
    let mut ret:Vec<u8> = Vec::new();
    ret.append( &mut sdb_config_page_start( &self.build_id, &self.nickname ));
    
    for gr in &self.graph_refs { ret.append( &mut sdb_config_graph_ref( gr ));}

    println!( "{:?} {:?}", page_size, ret.len() );
    
    ret.append( &mut sdb_config_page_end( &self.nickname ));
    ret
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_SDBConfigPage_new () 
  {
    let config: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "" ), 
      String::from( "" ),
      Vec::new() );
    assert_eq!( config.is_err(), true );
    assert_eq!( config, Err( DataGramError::InvalidDGUUID ));

    // ---
    let config1: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      String::from( "" ),
      Vec::new() );
    assert_eq!( config1.is_err(), true );
    assert_eq!( config1, Err( DataGramError::InvalidDGLabel ));

    // ---
    let config2: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      String::from( "nickname" ),
      Vec::new() );
    assert_eq!( config2.is_ok(), true );
    assert_eq!( config2.as_ref().unwrap().build_id, String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ));
    assert_eq!( config2.as_ref().unwrap().nickname, 
      pad_str( DGLabelBytes, String::from( "nickname" )));
  }

  #[test]
  fn test_SDBConfigPage_to_rows () 
  {
    let config_res: Result<SDBConfigPage, DataGramError> = SDBConfigPage::new( 
      String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" ), 
      String::from( "nickname" ),
      Vec::new() );
    assert_eq!( config_res.is_ok(), true );
    assert_eq!( config_res.unwrap().to_rows( &4096 ).len(), 196 );
  }
}