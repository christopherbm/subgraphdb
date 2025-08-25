use crate::common::validators::validate_label;
use crate::common::LABEL_BYTES;
use crate::datagramv2::external_grams::traits::KVP;
use crate::utils::gen_pad_str;


#[derive( Debug, Clone )]
pub struct KVPu8Gram { pub key: String, pub val: u8 }
impl KVPu8Gram 
{
  pub fn new ( key: String, val: u8 ) -> Result<KVPu8Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu8Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu8 invalid" ))
  }
}
impl KVP for KVPu8Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu16Gram { pub key: String, pub val: u16 }
impl KVPu16Gram 
{
  pub fn new ( key: String, val: u16 ) -> Result<KVPu16Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu16Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu16 invalid" ))
  }
}
impl KVP for KVPu16Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu32Gram { pub key: String, pub val: u32 }
impl KVPu32Gram 
{
  pub fn new ( key: String, val: u32 ) -> Result<KVPu32Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu32Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu32 invalid" ))
  }
}
impl KVP for KVPu32Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu64Gram { pub key: String, pub val: u64 }
impl KVPu64Gram 
{
  pub fn new ( key: String, val: u64 ) -> Result<KVPu64Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu64Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu64 invalid" ))
  }
}
impl KVP for KVPu64Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[derive( Debug, Clone )]
pub struct KVPu128Gram { pub key: String, pub val: u128 }
impl KVPu128Gram 
{
  pub fn new ( key: String, val: u128 ) -> Result<KVPu128Gram, String> 
  {
    if validate_label( &key ) 
    {
      return Ok( KVPu128Gram { key: key, val: val } );
    }
    Err( String::from( "KVPu128 invalid" ))
  }
}
impl KVP for KVPu128Gram
{
  fn unwrap ( &self ) -> Vec<u8> 
  {
    let key_padding = gen_pad_str( LABEL_BYTES - &self.key.bytes().len() );
    let mut ret = Vec::new();
    ret.append( &mut self.key.clone().into_bytes() );
    ret.append( &mut key_padding.into_bytes() );
    ret.append( &mut self.val.to_le_bytes().to_vec() );
    ret
  }
}


#[cfg(test)]
mod tests 
{
  use super::*;
  use crate::common::{ U128_BYTES, U16_BYTES, U32_BYTES, U64_BYTES, U8_BYTES };

  // KVPu8Gram Tests
  #[test]
  fn test_kvp_u8_gram_new_valid() 
  {
    let kvp = KVPu8Gram::new(String::from("age"), 25);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "age");
    assert_eq!(kvp_unwrapped.val, 25);
  }

  #[test]
  fn test_kvp_u8_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu8Gram::new(too_long_key, 100);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu8 invalid");
  }

  #[test]
  fn test_kvp_u8_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu8Gram::new(String::from("min"), u8::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu8Gram::new(String::from("max"), u8::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 255);
  }

  #[test]
  fn test_kvp_u8_gram_unwrap() 
  {
    let kvp = KVPu8Gram::new(String::from("score"), 200).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U8_BYTES);
  }

  #[test]
  fn test_kvp_u8_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu8Gram::new(max_key.clone(), 128);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U8_BYTES);
  }

  #[test]
  fn test_kvp_u8_gram_unicode_key() 
  {
    let kvp = KVPu8Gram::new(String::from("niño_age"), 8);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "niño_age");
    assert_eq!(kvp_unwrapped.val, 8);
  }


  // KVPu16Gram Tests
  #[test]
  fn test_kvp_u16_gram_new_valid() 
  {
    let kvp = KVPu16Gram::new(String::from("port"), 8080);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "port");
    assert_eq!(kvp_unwrapped.val, 8080);
  }

  #[test]
  fn test_kvp_u16_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu16Gram::new(too_long_key, 65535);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu16 invalid");
  }

  #[test]
  fn test_kvp_u16_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu16Gram::new(String::from("min"), u16::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu16Gram::new(String::from("max"), u16::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 65535);

    // Test common values
    let kvp_mid = KVPu16Gram::new(String::from("mid"), 32768);
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 32768);
  }

  #[test]
  fn test_kvp_u16_gram_unwrap() 
  {
    let kvp = KVPu16Gram::new(String::from("year"), 2024).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U16_BYTES);
  }

  #[test]
  fn test_kvp_u16_gram_unicode_key() 
  {
    let kvp = KVPu16Gram::new(String::from("año_count"), 2025);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "año_count");
    assert_eq!(kvp_unwrapped.val, 2025);
  }


  // KVPu32Gram Tests
  #[test]
  fn test_kvp_u32_gram_new_valid() 
  {
    let kvp = KVPu32Gram::new(String::from("population"), 1000000);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "population");
    assert_eq!(kvp_unwrapped.val, 1000000);
  }

  #[test]
  fn test_kvp_u32_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu32Gram::new(too_long_key, 4294967295);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu32 invalid");
  }

  #[test]
  fn test_kvp_u32_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu32Gram::new(String::from("min"), u32::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu32Gram::new(String::from("max"), u32::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 4294967295);

    // Test common boundary values
    let kvp_mid = KVPu32Gram::new(String::from("mid"), 2147483648); // 2^31
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 2147483648);
  }

  #[test]
  fn test_kvp_u32_gram_unwrap() 
  {
    let kvp = KVPu32Gram::new(String::from("bytes_count"), 1073741824).unwrap(); // 1GB in bytes
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U32_BYTES);
  }


  #[test]
  fn test_kvp_u32_gram_unicode_key() 
  {
    let kvp = KVPu32Gram::new(String::from("размер_bytes"), 2048000);
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "размер_bytes");
    assert_eq!(kvp_unwrapped.val, 2048000);
  }


  // KVPu64Gram Tests
  #[test]
  fn test_kvp_u64_gram_new_valid() 
  {
    let kvp = KVPu64Gram::new(String::from("file_size"), 1099511627776); // 1TB in bytes
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "file_size");
    assert_eq!(kvp_unwrapped.val, 1099511627776);
  }

  #[test]
  fn test_kvp_u64_gram_new_invalid_key() 
  {
    let too_long_key = "a".repeat(LABEL_BYTES + 1);
    let kvp = KVPu64Gram::new(too_long_key, 18446744073709551615);
    assert!(kvp.is_err());
    assert_eq!(kvp.unwrap_err(), "KVPu64 invalid");
  }

  #[test]
  fn test_kvp_u64_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu64Gram::new(String::from("min"), u64::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu64Gram::new(String::from("max"), u64::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 18446744073709551615);

    // Test common boundary values
    let kvp_mid = KVPu64Gram::new(String::from("mid"), 9223372036854775808); // 2^63
    assert!(kvp_mid.is_ok());
    assert_eq!(kvp_mid.as_ref().unwrap().val, 9223372036854775808);

    // Test large practical values
    let kvp_petabyte = KVPu64Gram::new(String::from("petabyte"), 1125899906842624); // 1PB in bytes
    assert!(kvp_petabyte.is_ok());
    assert_eq!(kvp_petabyte.as_ref().unwrap().val, 1125899906842624);
  }

  #[test]
  fn test_kvp_u64_gram_unwrap() 
  {
    let kvp = KVPu64Gram::new(String::from("timestamp"), 1735689600000).unwrap(); // Timestamp in milliseconds
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U64_BYTES);
  }

  #[test]
  fn test_kvp_u64_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu64Gram::new(max_key.clone(), 9999999999999999999);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U64_BYTES);
  }

  #[test]
  fn test_kvp_u64_gram_unicode_key() 
  {
    let kvp = KVPu64Gram::new(String::from("容量_bytes"), 5368709120); // 5GB in bytes
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "容量_bytes");
    assert_eq!(kvp_unwrapped.val, 5368709120);
  }

  
  // KVPu128Gram Tests
  #[test]
  fn test_kvp_u128_gram_new_valid() 
  {
    let kvp = KVPu128Gram::new(String::from("large_number"), 340282366920938463463374607431768211455); // u128::MAX
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "large_number");
    assert_eq!(kvp_unwrapped.val, 340282366920938463463374607431768211455);
  }

  #[test]
  fn test_kvp_u128_gram_edge_values() 
  {
    // Test minimum value
    let kvp_min = KVPu128Gram::new(String::from("min"), u128::MIN);
    assert!(kvp_min.is_ok());
    assert_eq!(kvp_min.as_ref().unwrap().val, 0);

    // Test maximum value
    let kvp_max = KVPu128Gram::new(String::from("max"), u128::MAX);
    assert!(kvp_max.is_ok());
    assert_eq!(kvp_max.as_ref().unwrap().val, 340282366920938463463374607431768211455);

    // Test powers of 2 boundaries
    let kvp_2_64 = KVPu128Gram::new(String::from("pow_64"), 1u128 << 64);
    assert!(kvp_2_64.is_ok());
    assert_eq!(kvp_2_64.as_ref().unwrap().val, 18446744073709551616);

    let kvp_2_127 = KVPu128Gram::new(String::from("pow_127"), 1u128 << 127);
    assert!(kvp_2_127.is_ok());
    assert_eq!(kvp_2_127.as_ref().unwrap().val, 170141183460469231731687303715884105728);
  }

  #[test]
  fn test_kvp_u128_gram_unwrap() 
  {
    let kvp = KVPu128Gram::new(String::from("uuid_num"), 12345678901234567890123456789012345678).unwrap();
    let unwrapped = kvp.unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U128_BYTES);
  }

  #[test]
  fn test_kvp_u128_gram_max_length_key() 
  {
    let max_key = "a".repeat(LABEL_BYTES);
    let kvp = KVPu128Gram::new(max_key.clone(), u128::MAX);
    assert!(kvp.is_ok());
    let unwrapped = kvp.unwrap().unwrap();
    assert_eq!(unwrapped.len(), LABEL_BYTES + U128_BYTES);
  }

  #[test]
  fn test_kvp_u128_gram_unicode_key() 
  {
    let kvp = KVPu128Gram::new(String::from("巨大数字_value"), 999999999999999999999999999999999999); 
    assert!(kvp.is_ok());
    let kvp_unwrapped = kvp.unwrap();
    assert_eq!(kvp_unwrapped.key, "巨大数字_value");
    assert_eq!(kvp_unwrapped.val, 999999999999999999999999999999999999);
  }
}