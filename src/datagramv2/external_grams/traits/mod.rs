/// Key-Value Pair
pub trait KVP 
{
  fn unwrap (&self) -> Vec<u8>;
}