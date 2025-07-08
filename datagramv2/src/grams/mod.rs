use utils::{ gen_pad_str };
use common::{ RAW_UUID_BYTES };

#[derive( Debug, Clone )]
pub struct Label { pub val: String, max_len: usize }
impl Label 
{
  pub fn new ( val: String, max_len: usize ) -> Result<Label, String> 
  {
    if Label::validate_label( &val, max_len ) 
    {
      return Ok( Label { val: val, max_len: max_len } );
    }
    Err( String::from( "Error creating Label" ))
  }

  /// Creates padded version of label
  pub fn unwrap ( &self ) -> String 
  {
    let padding = gen_pad_str( &self.max_len - &self.val.bytes().len() );
    self.val.clone() + &padding
  }

  pub fn validate_label ( val: &str, max_len: usize ) -> bool 
  {
    let size: usize = val.bytes().len();
    if size <= max_len { return true; }
    false
  }
}

#[derive( Debug, Clone )]
pub struct UUID { pub val: String }
impl UUID 
{
  pub fn new ( val: String ) -> Result<UUID, String> 
  {
    if UUID::validate_uuid( &val ) 
    {
      return Ok( UUID { val: val } )
    }
    Err( String::from( "Error creating UUID" ))
  }

  /// Creates padded version of uuid
  pub fn unwrap ( &self ) -> String { self.val.clone() + "::::" }

  pub fn validate_uuid ( val: &str ) -> bool
  {
    if val.bytes().len() == RAW_UUID_BYTES { return true; }
    return false;
  }
}

#[derive( Debug )]
pub struct DGu64 { pub val: u64 }
impl DGu64 
{
  pub fn new ( val: u64 ) -> DGu64 { DGu64 { val: val }}
  pub fn unwrap ( &self ) -> Vec<u8> { self.val.to_be_bytes().to_vec() }
}

// key value pair string
// key value pair float
// key value pair etc

#[cfg(test)]
mod tests 
{
  use super::*;
  use common::{ LABEL_BYTES };

  #[test]
  fn test_validate_label () 
  {
    let label = String::from( "devs" );
    assert_eq!( Label::validate_label( &label, LABEL_BYTES ), true );

    let label1 = String::from( "devs\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" );
    assert_eq!( Label::validate_label( &label1, LABEL_BYTES ), true );

    let label2 = String::from( "devs\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::F" );
    assert_eq!( Label::validate_label( &label2, LABEL_BYTES ), false );
  }

  #[test]
  fn test_label () 
  {
    let label = Label::new( String::from( "devs" ), LABEL_BYTES );
    assert_eq!( label.is_ok(), true );
    assert_eq!( label.unwrap().unwrap(), 
      String::from( "devs\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::::" ));
  }

  #[test]
  fn test_uuid () 
  {
    let raw_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" );
    let uuid_res = UUID::new( raw_uuid );
    assert_eq!( uuid_res.is_ok(), true );
    assert_eq!( uuid_res.unwrap().unwrap(), String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8::::" ));
  }

  #[test]
  fn test_validate_uuid () 
  {
    let raw_uuid = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8" );
    assert_eq!( UUID::validate_uuid( &raw_uuid ), true );

    // ---

    let raw_uuid1 = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c" );
    assert_eq!( UUID::validate_uuid( &raw_uuid1 ), false );

    // ---

    let raw_uuid2 = String::from( "67e55044-10b1-426f-9247-bb680e5fe0c8:" );
    assert_eq!( UUID::validate_uuid( &raw_uuid2 ), false );
  }
}