use crate::common::validators::validate_label;
use crate::common::{ FALSE_AFFIX, LABEL_BYTES, TRUE_AFFIX };
use crate::datagramv2::external_grams::traits::KVP;
use crate::utils::gen_pad_str;
/*
  Both the key and value are Label-length strings.
  unwrap -> LabelLabel
*/
#[derive( Debug, Clone )]
pub struct KVPStringGram { pub key: String, pub val: String }
impl KVPStringGram 
{
  pub fn new ( key: String, val: String ) -> Result<KVPStringGram, String> 
  {
    if validate_label( &key ) && validate_label( &val ) 
    {
      return Ok( KVPStringGram { key: key, val: val } );
    }
    Err( String::from( "KVPString invalid" ))
  }
}
impl KVP for KVPStringGram 
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let val_padding = gen_pad_str( LABEL_BYTES - &self.val.bytes().len() );
    ( self.key.clone() + &key_padding + &self.val.clone() + &val_padding ).into_bytes()
  }
}

#[derive( Debug, Clone )]
pub struct KVPBooleanGram { pub key: String, pub val: bool }
impl KVPBooleanGram 
{
  pub fn new ( key: String, val: bool ) -> Result<KVPBooleanGram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPBooleanGram { key: key, val: val } );
    }
    Err( String::from( "KVPBoolean invalid" ))
  }
}
impl KVP for KVPBooleanGram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let bool_str = if self.val { TRUE_AFFIX } else { FALSE_AFFIX };
    ( self.key.clone() + &key_padding + bool_str ).into_bytes()
  }
}

#[cfg(test)]
mod tests 
{
  use crate::common::ROW_AFFIX_BYTES;

use super::*;

  // KVPStringGram Tests
  #[test]
  fn test_kvp_string_gram_new () 
  {
    let kvp = KVPStringGram::new( String::from("name"), String::from("Alice") );
    assert!( kvp.is_ok() );
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "name");
    assert_eq!(kvp_unwrapped.val, "Alice");
  }

  #[test]
  fn test_kvp_string_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPStringGram::new(too_long_key, String::from("value"));
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPString invalid");
  }

  #[test]
  fn test_kvp_string_gram_new_invalid_val() 
  {
    let too_long_val = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPStringGram::new(String::from("key"), too_long_val);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPString invalid");
  }

  #[test]
  fn test_kvp_string_gram_unwrap() 
  {
    let kvp = KVPStringGram::new(String::from("test"), String::from("value")).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2); // Key + Value, both padded to LABEL_BYTES
  }

  #[test]
  fn test_kvp_string_gram_empty_strings() 
  {
    let kvp = KVPStringGram::new(String::from(""), String::from(""));
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2);
  }

  #[test]
  fn test_kvp_string_gram_unicode() 
  {
    let kvp = KVPStringGram::new(String::from("café"), String::from("niño"));
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "café");
    assert_eq!(kvp_unwrapped.val, "niño");
    let unwrapped = kvp_unwrapped.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES * 2);
  }

  // KVPBooleanGram Tests
  #[test]
  fn test_kvp_boolean_gram_new_valid() 
  {
    let kvp_true = KVPBooleanGram::new(String::from("active"), true);
    assert!(kvp_true.is_ok());
    let kvp_true_unwrapped = kvp_true.unwrap();
    assert_eq!(kvp_true_unwrapped.key, "active");
    assert_eq!(kvp_true_unwrapped.val, true);

    let kvp_false = KVPBooleanGram::new(String::from("enabled"), false);
    assert!(kvp_false.is_ok());
    let kvp_false_unwrapped = kvp_false.unwrap();
    assert_eq!(kvp_false_unwrapped.key, "enabled");
    assert_eq!(kvp_false_unwrapped.val, false);
  }

  #[test]
  fn test_kvp_boolean_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPBooleanGram::new(too_long_key, true);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPBoolean invalid");
  }

  #[test]
  fn test_kvp_boolean_gram_unwrap_true() 
  {
    let kvp = KVPBooleanGram::new(String::from("isActive"), true).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!( unwrapped.len(), LABEL_BYTES + ROW_AFFIX_BYTES );
  }

  #[test]
  fn test_kvp_boolean_gram_unwrap_false() 
  {
    let kvp = KVPBooleanGram::new(String::from("isEnabled"), false).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!( unwrapped.len(), LABEL_BYTES + ROW_AFFIX_BYTES );
  }
}