pub static DGUUIDBYTES: usize = 36 as usize;
pub static DGLABEL_BYTES: usize = 64 as usize; // names, labels, etc
pub static DGU64_BYTESs: usize = 8 as usize;
pub static DGSHORT_STRING_BYTES: usize = 8 as usize;

pub static COMMON_NONE: &'static str = "\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::NONE::";

pub static SDBConfigPageStartBytes: usize  = 124 as usize;
pub static SDBConfigGraphInfoBytes: usize  = 88 as usize;
pub static SDBConfigPageEndBytes: usize  = 88 as usize;

#[derive( Clone, Debug, PartialEq )]
pub enum DataGramError 
{
  InvalidDGUUID,
  InvalidDGCommonString,
  InvalidDGLabel,
  StringLengthExceeded,
  PaddingCannotBeZero,
  InvalidStringTerminus,
  InvalidSDBConfig,

  BuildIDEmpty,
  NicknameEmpty,
  ErrorWritingSDBConfig,
  ErrorReadingSDBConfig,
  ErrorCreatingSDBConfig,
}

/// Data Gram Types
#[derive( Debug, PartialEq )]
pub enum DataGramType
{
  DGUUID,
  DGCommonString,
  DGLabel,
}