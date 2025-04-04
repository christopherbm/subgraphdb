/// Database Trait
pub trait Database
{
  fn init( &self ) -> Result<bool, &'static str>;
  fn create ( &self ) -> Result<bool, &'static str>;
  fn load ( &self ) -> Result<bool, &'static str>;
}