use crate::common::validators::{ validate_label, validate_uuid };
use crate::common::LABEL_BYTES;
use crate::utils::{ cons_uuid, gen_pad_str };

#[derive( Debug, Clone )]
pub struct Label { pub val: String }
impl Label 
{
  pub fn new ( val: String ) -> Result<Label, String> 
  {
    if validate_label( &val ) 
    {
      return Ok( Label { val: val } );
    }
    Err( String::from( "Error creating Label" ))
  }

  /// Creates padded version of label
  pub fn unwrap ( &self ) -> String 
  {
    let padding = gen_pad_str( LABEL_BYTES - &self.val.bytes().len() );
    self.val.clone() + &padding
  }
}

#[derive( Debug, Clone )]
pub struct UUID { pub val: String }
impl UUID 
{
  pub fn new ( val: String ) -> Result<UUID, String> 
  {
    if validate_uuid( &val ) 
    {
      return Ok( UUID { val: val } )
    }
    Err( String::from( "Error creating UUID" ))
  }

  pub fn generate () -> UUID { UUID::new( cons_uuid() ).unwrap() }

  /// Creates padded version of uuid
  pub fn unwrap ( &self ) -> String { self.val.clone() + "::::" }
}

#[derive( Debug )]
pub struct DGu64 { pub val: u64 }
impl DGu64 
{
  pub fn new ( val: u64 ) -> DGu64 { DGu64 { val: val }}
  pub fn unwrap ( &self ) -> Vec<u8> { self.val.to_be_bytes().to_vec() }
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_label_new_valid() 
  {
    // Test creating valid labels
    let label1 = Label::new(String::from("User"));
    assert!(label1.is_ok());
    assert_eq!(label1.unwrap().val, "User");

    let label2 = Label::new(String::from("Person"));
    assert!(label2.is_ok());
    assert_eq!(label2.unwrap().val, "Person");

    let label3 = Label::new(String::from(""));
    assert!(label3.is_ok());
    assert_eq!(label3.unwrap().val, "");
  }

  #[test]
  fn test_label_new_invalid() 
  {
    // Test creating labels that exceed LABEL_BYTES
    let too_long = "a".repeat(LABEL_BYTES + 1);
    let label = Label::new(too_long);
    assert!(label.is_err());
    assert_eq!(label.unwrap_err(), "Error creating Label");
  }

  #[test]
  fn test_label_new_exact_limit() 
  {
    // Test label at exact byte limit
    let exact = "a".repeat(LABEL_BYTES);
    let label = Label::new(exact.clone());
    assert!(label.is_ok());
    assert_eq!(label.unwrap().val, exact);
  }

  #[test]
  fn test_label_unwrap_padding() 
  {
    // Test unwrap adds proper padding
    let label = Label::new(String::from("Test")).unwrap();
    let padded = label.unwrap();
    assert_eq!(padded.len(), LABEL_BYTES);
    assert!(padded.starts_with("Test"));
    assert!(padded.ends_with(&gen_pad_str(LABEL_BYTES - 4)));
  }

  #[test]
  fn test_label_unwrap_empty() 
  {
    // Test unwrap with empty label
    let label = Label::new(String::from("")).unwrap();
    let padded = label.unwrap();
    assert_eq!(padded.len(), LABEL_BYTES);
    assert_eq!(padded, gen_pad_str(LABEL_BYTES));
  }

  #[test]
  fn test_label_unicode() 
  {
    // Test labels with unicode characters
    let label1 = Label::new(String::from("café"));
    assert!(label1.is_ok());
    assert_eq!(label1.as_ref().unwrap().val, "café");

    let label2 = Label::new(String::from("🚀"));
    assert!(label2.is_ok());
    assert_eq!(label2.as_ref().unwrap().val, "🚀");

    // Test unicode padding
    let padded = label2.unwrap().unwrap();
    assert_eq!(padded.len(), LABEL_BYTES);
  }

  #[test]
  fn test_uuid_new_valid() 
  {
    // Test creating valid UUID (36 bytes)
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = UUID::new(String::from(uuid_str));
    assert!(uuid.is_ok());
    assert_eq!(uuid.unwrap().val, uuid_str);
  }

  #[test]
  fn test_uuid_new_invalid_length() 
  {
    // Test creating UUID with wrong length
    let too_short = "550e8400-e29b-41d4-a716";
    let uuid1 = UUID::new(String::from(too_short));
    assert!(uuid1.is_err());
    assert_eq!(uuid1.unwrap_err(), "Error creating UUID");

    let too_long = "550e8400-e29b-41d4-a716-446655440000-extra";
    let uuid2 = UUID::new(String::from(too_long));
    assert!(uuid2.is_err());
    assert_eq!(uuid2.unwrap_err(), "Error creating UUID");
  }

  #[test]
  fn test_uuid_generate() 
  {
    // Test UUID generation
    let uuid1 = UUID::generate();
    assert_eq!(uuid1.val.len(), 36); // Standard UUID length
    
    let uuid2 = UUID::generate();
    assert_eq!(uuid2.val.len(), 36);
  }

  #[test]
  fn test_uuid_unwrap() 
  {
    // Test UUID unwrap adds padding
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let uuid = UUID::new(String::from(uuid_str)).unwrap();
    let padded = uuid.unwrap();
    assert_eq!(padded.len(), 40); // 36 + 4
  }

  #[test]
  fn test_dgu64_new() 
  {
    // Test creating DGu64
    let dg0 = DGu64::new(0);
    assert_eq!(dg0.val, 0);

    let dg1 = DGu64::new(42);
    assert_eq!(dg1.val, 42);

    let dg_max = DGu64::new(u64::MAX);
    assert_eq!(dg_max.val, u64::MAX);
  }

  #[test]
  fn test_dgu64_unwrap_edge_cases() 
  {
    // Test edge cases
    let dg_zero = DGu64::new(0);
    let bytes_zero = dg_zero.unwrap();
    assert_eq!(bytes_zero, vec![0, 0, 0, 0, 0, 0, 0, 0]);

    let dg_max = DGu64::new(u64::MAX);
    let bytes_max = dg_max.unwrap();
    assert_eq!(bytes_max, vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);

    let dg_one = DGu64::new(1);
    let bytes_one = dg_one.unwrap();
    assert_eq!(bytes_one, vec![0, 0, 0, 0, 0, 0, 0, 1]);
  }
}