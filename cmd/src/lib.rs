/// Insert Node Command
#[derive( Debug, PartialEq )]
pub struct InsertNode {}

/// Insert Edge Command
#[derive( Debug, PartialEq )]
pub struct InsertEdge {}

#[cfg(test)]
mod tests 
{
  use super::*;

  #[test]
  fn it_works () 
  {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}
