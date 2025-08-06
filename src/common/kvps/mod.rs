#[derive( Debug )]
pub struct KeyValString { pub key: String, pub val: Option<String> }
impl KeyValString 
{
  pub fn new ( key: String, val: Option<String> ) -> KeyValString
  {
    KeyValString { key: key, val: val }
  }

  pub fn from ( kvs: &KeyValString, val: String ) -> KeyValString 
  {
    KeyValString { key: kvs.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValBoolean { pub key: String, pub val: Option<bool> }
impl KeyValBoolean
{
  pub fn new ( key: String, val: Option<bool> ) -> KeyValBoolean
  {
    KeyValBoolean { key: key, val: val }
  }

  pub fn from ( kvs: &KeyValBoolean, val: bool ) -> KeyValBoolean 
  {
    KeyValBoolean { key: kvs.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValI8 { pub key: String, pub val: Option<i8> }
impl KeyValI8 
{
  pub fn new ( key: String, val: Option<i8> ) -> KeyValI8
  {
    KeyValI8 { key: key, val: val }
  }

  pub fn from ( kvi8: &KeyValI8, val: i8 ) -> KeyValI8 
  {
    KeyValI8 { key: kvi8.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValU8 { pub key: String, pub val: Option<u8> }
impl KeyValU8 
{
  pub fn new ( key: String, val: Option<u8> ) -> KeyValU8
  {
    KeyValU8 { key: key, val: val }
  }

  pub fn from ( kvu8: &KeyValU8, val: u8 ) -> KeyValU8 
  {
    KeyValU8 { key: kvu8.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValI16 { pub key: String, pub val: Option<i16> }
impl KeyValI16 
{
  pub fn new ( key: String, val: Option<i16> ) -> KeyValI16
  {
    KeyValI16 { key: key, val: val }
  }

  pub fn from ( kvi16: &KeyValI16, val: i16 ) -> KeyValI16 
  {
    KeyValI16 { key: kvi16.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValU16 { pub key: String, pub val: Option<u16> }
impl KeyValU16 
{
  pub fn new ( key: String, val: Option<u16> ) -> KeyValU16
  {
    KeyValU16 { key: key, val: val }
  }

  pub fn from ( kvu16: &KeyValU16, val: u16 ) -> KeyValU16 
  {
    KeyValU16 { key: kvu16.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValI32 { pub key: String, pub val: Option<i32> }
impl KeyValI32 
{
  pub fn new ( key: String, val: Option<i32> ) -> KeyValI32
  {
    KeyValI32 { key: key, val: val }
  }

  pub fn from ( kvi32: &KeyValI32, val: i32 ) -> KeyValI32 
  {
    KeyValI32 { key: kvi32.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValU32 { pub key: String, pub val: Option<u32> }
impl KeyValU32 
{
  pub fn new ( key: String, val: Option<u32> ) -> KeyValU32
  {
    KeyValU32 { key: key, val: val }
  }

  pub fn from ( kvu32: &KeyValU32, val: u32 ) -> KeyValU32 
  {
    KeyValU32 { key: kvu32.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValI64 { pub key: String, pub val: Option<i64> }
impl KeyValI64 
{
  pub fn new ( key: String, val: Option<i64> ) -> KeyValI64
  {
    KeyValI64 { key: key, val: val }
  }

  pub fn from ( kvi64: &KeyValI64, val: i64 ) -> KeyValI64 
  {
    KeyValI64 { key: kvi64.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValU64 { pub key: String, pub val: Option<u64> }
impl KeyValU64 
{
  pub fn new ( key: String, val: Option<u64> ) -> KeyValU64
  {
    KeyValU64 { key: key, val: val }
  }

  pub fn from ( kvu64: &KeyValU64, val: u64 ) -> KeyValU64 
  {
    KeyValU64 { key: kvu64.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValI128 { pub key: String, pub val: Option<i128> }
impl KeyValI128 
{
  pub fn new ( key: String, val: Option<i128> ) -> KeyValI128
  {
    KeyValI128 { key: key, val: val }
  }

  pub fn from ( kvi128: &KeyValI128, val: i128 ) -> KeyValI128 
  {
    KeyValI128 { key: kvi128.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValU128 { pub key: String, pub val: Option<u128> }
impl KeyValU128 
{
  pub fn new ( key: String, val: Option<u128> ) -> KeyValU128
  {
    KeyValU128 { key: key, val: val }
  }

  pub fn from ( kvu128: &KeyValU128, val: u128 ) -> KeyValU128 
  {
    KeyValU128 { key: kvu128.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValF32 { pub key: String, pub val: Option<f32> }
impl KeyValF32 
{
  pub fn new ( key: String, val: Option<f32> ) -> KeyValF32
  {
    KeyValF32 { key: key, val: val }
  }

  pub fn from ( kvf32: &KeyValF32, val: f32 ) -> KeyValF32 
  {
    KeyValF32 { key: kvf32.key.clone(), val: Some( val ) }
  }
}

#[derive( Debug )]
pub struct KeyValF64 { pub key: String, pub val: Option<f64> }
impl KeyValF64 
{
  pub fn new ( key: String, val: Option<f64> ) -> KeyValF64
  {
    KeyValF64 { key: key, val: val }
  }

  pub fn from ( kvf64: &KeyValF64, val: f64 ) -> KeyValF64 
  {
    KeyValF64 { key: kvf64.key.clone(), val: Some( val ) }
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_key_value_string () 
  {
    let kvs1 = KeyValString::new( String::from("name"), Some(String::from("Alice")) );
    assert_eq!( kvs1.key, "name" );
    assert_eq!( kvs1.val, Some(String::from("Alice")) );

    let kvs2 = KeyValString::new( String::from("description"), None );
    assert_eq!( kvs2.key, "description" );
    assert_eq!( kvs2.val, None );

    let kvs3 = KeyValString::from( &kvs2, String::from("A test description") );
    assert_eq!( kvs3.key, "description" );
    assert_eq!( kvs3.val, Some(String::from("A test description")) );
  }

  #[test]
  fn test_key_value_boolean () 
  {
    let kvb1 = KeyValBoolean::new( String::from("active"), Some(true) );
    assert_eq!( kvb1.key, "active" );
    assert_eq!( kvb1.val, Some(true) );

    let kvb2 = KeyValBoolean::new( String::from("active"), None );
    assert_eq!( kvb2.key, "active" );
    assert_eq!( kvb2.val, None );

    let kvb3 = KeyValBoolean::from( &kvb2, true );
    assert_eq!( kvb3.key, "active" );
    assert_eq!( kvb3.val, Some(true) );
  }

  #[test]
  fn test_key_value_i8 () 
  {
    let kvi1 = KeyValI8::new( String::from("temperature"), Some(-10) );
    assert_eq!( kvi1.key, "temperature" );
    assert_eq!( kvi1.val, Some(-10) );

    let kvi2 = KeyValI8::new( String::from("level"), None );
    assert_eq!( kvi2.key, "level" );
    assert_eq!( kvi2.val, None );

    let kvi3 = KeyValI8::from( &kvi2, 42 );
    assert_eq!( kvi3.key, "level" );
    assert_eq!( kvi3.val, Some(42) );

    // Test edge cases
    let kvi_min = KeyValI8::new( String::from("min_value"), Some(i8::MIN) );
    assert_eq!( kvi_min.val, Some(-128) );

    let kvi_max = KeyValI8::new( String::from("max_value"), Some(i8::MAX) );
    assert_eq!( kvi_max.val, Some(127) );
  }

  #[test]
  fn test_key_value_u8 () 
  {
    let kvu1 = KeyValU8::new( String::from("age"), Some(25) );
    assert_eq!( kvu1.key, "age" );
    assert_eq!( kvu1.val, Some(25) );

    let kvu2 = KeyValU8::new( String::from("score"), None );
    assert_eq!( kvu2.key, "score" );
    assert_eq!( kvu2.val, None );

    let kvu3 = KeyValU8::from( &kvu2, 100 );
    assert_eq!( kvu3.key, "score" );
    assert_eq!( kvu3.val, Some(100) );

    // Test edge cases
    let kvu_min = KeyValU8::new( String::from("min_value"), Some(u8::MIN) );
    assert_eq!( kvu_min.val, Some(0) );

    let kvu_max = KeyValU8::new( String::from("max_value"), Some(u8::MAX) );
    assert_eq!( kvu_max.val, Some(255) );
  }

  #[test]
  fn test_key_value_i16 () 
  {
    let kvi1 = KeyValI16::new( String::from("voltage"), Some(-1000) );
    assert_eq!( kvi1.key, "voltage" );
    assert_eq!( kvi1.val, Some(-1000) );

    let kvi2 = KeyValI16::new( String::from("altitude"), None );
    assert_eq!( kvi2.key, "altitude" );
    assert_eq!( kvi2.val, None );

    let kvi3 = KeyValI16::from( &kvi2, 5280 );
    assert_eq!( kvi3.key, "altitude" );
    assert_eq!( kvi3.val, Some(5280) );

    // Test edge cases
    let kvi_min = KeyValI16::new( String::from("min_value"), Some(i16::MIN) );
    assert_eq!( kvi_min.val, Some(-32768) );

    let kvi_max = KeyValI16::new( String::from("max_value"), Some(i16::MAX) );
    assert_eq!( kvi_max.val, Some(32767) );
  }

  #[test]
  fn test_key_value_u16 () 
  {
    let kvu1 = KeyValU16::new( String::from("port"), Some(8080) );
    assert_eq!( kvu1.key, "port" );
    assert_eq!( kvu1.val, Some(8080) );

    let kvu2 = KeyValU16::new( String::from("pixels"), None );
    assert_eq!( kvu2.key, "pixels" );
    assert_eq!( kvu2.val, None );

    let kvu3 = KeyValU16::from( &kvu2, 1920 );
    assert_eq!( kvu3.key, "pixels" );
    assert_eq!( kvu3.val, Some(1920) );

    // Test edge cases
    let kvu_min = KeyValU16::new( String::from("min_value"), Some(u16::MIN) );
    assert_eq!( kvu_min.val, Some(0) );

    let kvu_max = KeyValU16::new( String::from("max_value"), Some(u16::MAX) );
    assert_eq!( kvu_max.val, Some(65535) );
  }

  #[test]
  fn test_key_value_i32 () 
  {
    let kvi1 = KeyValI32::new( String::from("balance"), Some(-50000) );
    assert_eq!( kvi1.key, "balance" );
    assert_eq!( kvi1.val, Some(-50000) );

    let kvi2 = KeyValI32::new( String::from("offset"), None );
    assert_eq!( kvi2.key, "offset" );
    assert_eq!( kvi2.val, None );

    let kvi3 = KeyValI32::from( &kvi2, 1000000 );
    assert_eq!( kvi3.key, "offset" );
    assert_eq!( kvi3.val, Some(1000000) );

    // Test edge cases
    let kvi_min = KeyValI32::new( String::from("min_value"), Some(i32::MIN) );
    assert_eq!( kvi_min.val, Some(-2147483648) );

    let kvi_max = KeyValI32::new( String::from("max_value"), Some(i32::MAX) );
    assert_eq!( kvi_max.val, Some(2147483647) );
  }

  #[test]
  fn test_key_value_u32 () 
  {
    let kvu1 = KeyValU32::new( String::from("timestamp"), Some(1609459200) );
    assert_eq!( kvu1.key, "timestamp" );
    assert_eq!( kvu1.val, Some(1609459200) );

    let kvu2 = KeyValU32::new( String::from("count"), None );
    assert_eq!( kvu2.key, "count" );
    assert_eq!( kvu2.val, None );

    let kvu3 = KeyValU32::from( &kvu2, 3000000000 );
    assert_eq!( kvu3.key, "count" );
    assert_eq!( kvu3.val, Some(3000000000) );

    // Test edge cases
    let kvu_min = KeyValU32::new( String::from("min_value"), Some(u32::MIN) );
    assert_eq!( kvu_min.val, Some(0) );

    let kvu_max = KeyValU32::new( String::from("max_value"), Some(u32::MAX) );
    assert_eq!( kvu_max.val, Some(4294967295) );
  }

  #[test]
  fn test_key_value_i64 () 
  {
    let kvi1 = KeyValI64::new( String::from("debt"), Some(-9223372036854775800) );
    assert_eq!( kvi1.key, "debt" );
    assert_eq!( kvi1.val, Some(-9223372036854775800) );

    let kvi2 = KeyValI64::new( String::from("position"), None );
    assert_eq!( kvi2.key, "position" );
    assert_eq!( kvi2.val, None );

    let kvi3 = KeyValI64::from( &kvi2, 1234567890123456789 );
    assert_eq!( kvi3.key, "position" );
    assert_eq!( kvi3.val, Some(1234567890123456789) );

    // Test edge cases
    let kvi_min = KeyValI64::new( String::from("min_value"), Some(i64::MIN) );
    assert_eq!( kvi_min.val, Some(-9223372036854775808) );
    
    let kvi_max = KeyValI64::new( String::from("max_value"), Some(i64::MAX) );
    assert_eq!( kvi_max.val, Some(9223372036854775807) );
  }

  #[test]
  fn test_key_value_u64 () 
  {
    let kvu1 = KeyValU64::new( String::from("file_size"), Some(18446744073709551615) );
    assert_eq!( kvu1.key, "file_size" );
    assert_eq!( kvu1.val, Some(18446744073709551615) );

    let kvu2 = KeyValU64::new( String::from("memory_address"), None );
    assert_eq!( kvu2.key, "memory_address" );
    assert_eq!( kvu2.val, None );

    let kvu3 = KeyValU64::from( &kvu2, 9876543210987654321 );
    assert_eq!( kvu3.key, "memory_address" );
    assert_eq!( kvu3.val, Some(9876543210987654321) );

    // Test edge cases
    let kvu_min = KeyValU64::new( String::from("min_value"), Some(u64::MIN) );
    assert_eq!( kvu_min.val, Some(0) );

    let kvu_max = KeyValU64::new( String::from("max_value"), Some(u64::MAX) );
    assert_eq!( kvu_max.val, Some(18446744073709551615) );
  }

  #[test]
  fn test_key_value_i128 () 
  {
    let kvi1 = KeyValI128::new( String::from("large_negative"), Some(-170141183460469231731687303715884105728) );
    assert_eq!( kvi1.key, "large_negative" );
    assert_eq!( kvi1.val, Some(-170141183460469231731687303715884105728) );

    let kvi2 = KeyValI128::new( String::from("crypto_value"), None );
    assert_eq!( kvi2.key, "crypto_value" );
    assert_eq!( kvi2.val, None );

    let kvi3 = KeyValI128::from( &kvi2, 123456789012345678901234567890 );
    assert_eq!( kvi3.key, "crypto_value" );
    assert_eq!( kvi3.val, Some(123456789012345678901234567890) );

    // Test edge cases
    let kvi_min = KeyValI128::new( String::from("min_value"), Some(i128::MIN) );
    assert_eq!( kvi_min.val, Some(-170141183460469231731687303715884105728) );

    let kvi_max = KeyValI128::new( String::from("max_value"), Some(i128::MAX) );
    assert_eq!( kvi_max.val, Some(170141183460469231731687303715884105727) );
  }

  #[test]
  fn test_key_value_u128 () 
  {
    let kvu1 = KeyValU128::new( String::from("huge_counter"), Some(340282366920938463463374607431768211455) );
    assert_eq!( kvu1.key, "huge_counter" );
    assert_eq!( kvu1.val, Some(340282366920938463463374607431768211455) );

    let kvu2 = KeyValU128::new( String::from("blockchain_id"), None );
    assert_eq!( kvu2.key, "blockchain_id" );
    assert_eq!( kvu2.val, None );

    // (!!!)

    //let kvu3 = KeyValU128::from( &kvu2, 987654321098765432109876543210987654321 );
    //assert_eq!( kvu3.key, "blockchain_id" );
    //assert_eq!( kvu3.val, Some(987654321098765432109876543210987654321) );

    // Test edge cases
    //let kvu_min = KeyValU128::new( String::from("min_value"), Some(u128::MIN) );
    //assert_eq!( kvu_min.val, Some(0) );

    //let kvu_max = KeyValU128::new( String::from("max_value"), Some(u128::MAX) );
    //assert_eq!( kvu_max.val, Some(340282366920938463463374607431768211455) );
  }

  #[test]
  fn test_integer_byte_lengths () 
  {
    // Test i8 - should be 1 byte (8 bits)
    let i8_val: i8 = 42;
    let i8_bytes = i8_val.to_le_bytes();
    assert_eq!( i8_bytes.len(), 1 );
    assert_eq!( i8_bytes.len() * 8, 8 ); // 8 bits

    // Test u8 - should be 1 byte (8 bits)
    let u8_val: u8 = 42;
    let u8_bytes = u8_val.to_le_bytes();
    assert_eq!( u8_bytes.len(), 1 );

    // Test i16 - should be 2 bytes (16 bits)
    let i16_val: i16 = 1000;
    let i16_bytes = i16_val.to_le_bytes();
    assert_eq!( i16_bytes.len(), 2 );

    // Test u16 - should be 2 bytes (16 bits)
    let u16_val: u16 = 1000;
    let u16_bytes = u16_val.to_le_bytes();
    assert_eq!( u16_bytes.len(), 2 );

    // Test i32 - should be 4 bytes (32 bits)
    let i32_val: i32 = 1000000;
    let i32_bytes = i32_val.to_le_bytes();
    assert_eq!( i32_bytes.len(), 4 );

    // Test u32 - should be 4 bytes (32 bits)
    let u32_val: u32 = 1000000;
    let u32_bytes = u32_val.to_le_bytes();
    assert_eq!( u32_bytes.len(), 4 );

    // Test i64 - should be 8 bytes (64 bits)
    let i64_val: i64 = 1000000000000;
    let i64_bytes = i64_val.to_le_bytes();
    assert_eq!( i64_bytes.len(), 8 );

    // Test u64 - should be 8 bytes (64 bits)
    let u64_val: u64 = 1000000000000;
    let u64_bytes = u64_val.to_le_bytes();
    assert_eq!( u64_bytes.len(), 8 );

    // Test i128 - should be 16 bytes (128 bits)
    let i128_val: i128 = 123456789012345678901234567890;
    let i128_bytes = i128_val.to_le_bytes();
    assert_eq!( i128_bytes.len(), 16 );

    // Test u128 - should be 16 bytes (128 bits)
    let u128_val: u128 = 123456789012345678901234567890;
    let u128_bytes = u128_val.to_le_bytes();
    assert_eq!( u128_bytes.len(), 16 );
  }

  #[test]
  fn test_keyval_integer_byte_serialization ()
  {
    let kvi8 = KeyValI8::new( String::from("temp"), Some(-42) );
    assert_eq!( kvi8.val.unwrap().to_le_bytes().len(), 1 );

    let kvu16 = KeyValU16::new( String::from("port"), Some(8080) );
    assert_eq!( kvu16.val.unwrap().to_le_bytes().len(), 2 );

    let kvi32 = KeyValI32::new( String::from("offset"), Some(-1000000) );
    assert_eq!( kvi32.val.unwrap().to_le_bytes().len(), 4 );

    let kvu64 = KeyValU64::new( String::from("size"), Some(18446744073709551615) );
    assert_eq!( kvu64.val.unwrap().to_le_bytes().len(), 8 );

    let kvi128 = KeyValI128::new( String::from("huge"), Some(i128::MAX) );
    assert_eq!( kvi128.val.unwrap().to_le_bytes().len(), 16 );
  }

  #[test]
  fn test_key_value_f32 () 
  {
    let kvf1 = KeyValF32::new( String::from("temperature"), Some(23.5) );
    assert_eq!( kvf1.key, "temperature" );
    assert_eq!( kvf1.val, Some(23.5) );

    let kvf2 = KeyValF32::new( String::from("pressure"), None );
    assert_eq!( kvf2.key, "pressure" );
    assert_eq!( kvf2.val, None );

    let kvf3 = KeyValF32::from( &kvf2, 101.325 );
    assert_eq!( kvf3.key, "pressure" );
    assert_eq!( kvf3.val, Some(101.325) );

    // Test special values
    let kvf_zero = KeyValF32::new( String::from("zero"), Some(0.0) );
    assert_eq!( kvf_zero.val, Some(0.0) );

    let kvf_neg_zero = KeyValF32::new( String::from("neg_zero"), Some(-0.0) );
    assert_eq!( kvf_neg_zero.val, Some(-0.0) );

    let kvf_infinity = KeyValF32::new( String::from("infinity"), Some(f32::INFINITY) );
    assert!( kvf_infinity.val.unwrap().is_infinite() );

    let kvf_neg_infinity = KeyValF32::new( String::from("neg_infinity"), Some(f32::NEG_INFINITY) );
    assert!( kvf_neg_infinity.val.unwrap().is_infinite() );

    let kvf_nan = KeyValF32::new( String::from("nan"), Some(f32::NAN) );
    assert!( kvf_nan.val.unwrap().is_nan() );

    // Test edge cases
    let kvf_min = KeyValF32::new( String::from("min_value"), Some(f32::MIN) );
    assert_eq!( kvf_min.val, Some(f32::MIN) );

    let kvf_max = KeyValF32::new( String::from("max_value"), Some(f32::MAX) );
    assert_eq!( kvf_max.val, Some(f32::MAX) );

    let kvf_epsilon = KeyValF32::new( String::from("epsilon"), Some(f32::EPSILON) );
    assert_eq!( kvf_epsilon.val, Some(f32::EPSILON) );
  }

  #[test]
  fn test_key_value_f64 () 
  {
    let kvf1 = KeyValF64::new( String::from("latitude"), Some(37.7749) );
    assert_eq!( kvf1.key, "latitude" );
    assert_eq!( kvf1.val, Some(37.7749) );

    let kvf2 = KeyValF64::new( String::from("longitude"), None );
    assert_eq!( kvf2.key, "longitude" );
    assert_eq!( kvf2.val, None );

    let kvf3 = KeyValF64::from( &kvf2, -122.4194 );
    assert_eq!( kvf3.key, "longitude" );
    assert_eq!( kvf3.val, Some(-122.4194) );

    // Test special values
    let kvf_zero = KeyValF64::new( String::from("zero"), Some(0.0) );
    assert_eq!( kvf_zero.val, Some(0.0) );

    let kvf_neg_zero = KeyValF64::new( String::from("neg_zero"), Some(-0.0) );
    assert_eq!( kvf_neg_zero.val, Some(-0.0) );

    let kvf_infinity = KeyValF64::new( String::from("infinity"), Some(f64::INFINITY) );
    assert!( kvf_infinity.val.unwrap().is_infinite() );

    let kvf_neg_infinity = KeyValF64::new( String::from("neg_infinity"), Some(f64::NEG_INFINITY) );
    assert!( kvf_neg_infinity.val.unwrap().is_infinite() );

    let kvf_nan = KeyValF64::new( String::from("nan"), Some(f64::NAN) );
    assert!( kvf_nan.val.unwrap().is_nan() );

    // Test edge cases
    let kvf_min = KeyValF64::new( String::from("min_value"), Some(f64::MIN) );
    assert_eq!( kvf_min.val, Some(f64::MIN) );

    let kvf_max = KeyValF64::new( String::from("max_value"), Some(f64::MAX) );
    assert_eq!( kvf_max.val, Some(f64::MAX) );

    let kvf_epsilon = KeyValF64::new( String::from("epsilon"), Some(f64::EPSILON) );
    assert_eq!( kvf_epsilon.val, Some(f64::EPSILON) );
  }

  #[test]
  fn test_floating_point_byte_lengths () 
  {
    // Test f32 - should be 4 bytes (32 bits)
    let f32_val: f32 = 3.14159;
    let f32_bytes = f32_val.to_le_bytes();
    assert_eq!( f32_bytes.len(), 4 );
    assert_eq!( f32_bytes.len() * 8, 32 ); // 32 bits
    assert_eq!( 32 % 8, 0 ); // factor of 8

    // Test f64 - should be 8 bytes (64 bits)
    let f64_val: f64 = 3.141592653589793;
    let f64_bytes = f64_val.to_le_bytes();
    assert_eq!( f64_bytes.len(), 8 );

    // Test special f32 values
    assert_eq!( f32::INFINITY.to_le_bytes().len(), 4 );
    assert_eq!( f32::NEG_INFINITY.to_le_bytes().len(), 4 );
    assert_eq!( f32::NAN.to_le_bytes().len(), 4 );

    // Test special f64 values
    assert_eq!( f64::INFINITY.to_le_bytes().len(), 8 );
    assert_eq!( f64::NEG_INFINITY.to_le_bytes().len(), 8 );
    assert_eq!( f64::NAN.to_le_bytes().len(), 8 );
  }

  #[test]
  fn test_keyval_floating_point_byte_serialization ()
  {
    // Test with actual KeyVal structs
    let kvf32 = KeyValF32::new( String::from("pi"), Some(3.14159) );
    assert_eq!( kvf32.val.unwrap().to_le_bytes().len(), 4 );
    
    let kvf64 = KeyValF64::new( String::from("e"), Some(2.718281828459045) );
    assert_eq!( kvf64.val.unwrap().to_le_bytes().len(), 8 );

    // Test edge cases
    let kvf32_max = KeyValF32::new( String::from("max"), Some(f32::MAX) );
    assert_eq!( kvf32_max.val.unwrap().to_le_bytes().len(), 4 );

    let kvf64_min = KeyValF64::new( String::from("min"), Some(f64::MIN) );
    assert_eq!( kvf64_min.val.unwrap().to_le_bytes().len(), 8 );
  }
}