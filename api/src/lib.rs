/*
   /sdb/im/{nickname} (GPPD)
   /sdb/sf/{nickname} {GPPD}
   /sdb/mf/{nickname} {GPPD}

   /sdb/im/{nickname}/graph/{nickname} (GPPD)
   /sdb/sf/{nickname}/graph/{nickname} (GPPD)
   /sdb/mf/{nickname}/graph/{nickname} (GPPD)
*/

// load_db (db_type, path)
// create_db (db_type, path, Option<nickname>)

pub fn add(left: u64, right: u64) -> u64 
{
  left + right
}

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
