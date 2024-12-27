pub fn add(left: usize, right: usize) -> usize 
{
    left + right
}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn test_string_binary () 
  {
    let bytes: Vec<_> = b"test".to_vec();
    assert_eq!( bytes, [116, 101, 115, 116] );
  }
}
