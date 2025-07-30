pub mod constants;
pub mod rows;
pub mod dg_utils;
pub mod grams;

/* @version 0.3.0 */

pub fn add(left: u64, right: u64) -> u64 { left + right }

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn it_works() 
  {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
