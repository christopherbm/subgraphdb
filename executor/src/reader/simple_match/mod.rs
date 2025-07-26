use cmd::transaction::Transaction;

/*
MATCH () FROM devs;

MATCH (n)

MATCH (n) FROM devs RETURN n.name

MATCH (n:Stop)

MATCH (n:Developer)
FROM devs
RETURN n AS Developer
*/

pub struct SimpleMatchExecutor<'a> 
{
  pub transaction: &'a Transaction,
  pub path: &'a str,
  pub page_size: usize,
}

impl SimpleMatchExecutor<'_>
{
  pub fn new<'a> ( t: &'a Transaction, path: &'a str, page_size: usize ) -> SimpleMatchExecutor<'a>
  {
    SimpleMatchExecutor 
    {
      transaction: t, 
      path: path, 
      page_size: page_size,
    } 
  }
}

#[cfg(test)]
mod tests 
{
  use super::*;
}