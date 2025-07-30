use crate::cmd::transaction::Transaction;

pub struct UpdateGraphExecutor<'a>
{
  pub transaction: &'a Transaction,
  pub path: &'a str,
  pub page_size: usize,
}

impl UpdateGraphExecutor<'_>
{
  pub fn new<'a> ( t: &'a Transaction, path: &'a str, page_size: usize ) -> UpdateGraphExecutor<'a>
  {
    UpdateGraphExecutor 
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