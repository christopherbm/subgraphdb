pub static DGUUID_BYTES: usize = 40 as usize;
pub static DGLABEL_BYTES: usize = 64 as usize; // names, labels, etc
pub static DGU64_BYTES: usize = 8 as usize;
pub static DGSHORT_STRING_BYTES: usize = 8 as usize;
pub static ROW_PREFIX_BYTES: usize = 8 as usize;

pub static COMMON_NONE: &'static str = "\\:::::::::::::::::::::::::::::::::::::::::::::::::::::::::NONE::";
pub static EMPTY_BYTE: &'static str = "[::::::]";

pub static SDBCONFIG_PAGE_START_BYTES: usize  = 128 as usize;
pub static SDBCONFIG_PAGE_END_BYTES: usize  = 128 as usize;
pub static SDBCONFIG_GRAPH_REF_BYTES: usize  = 80 as usize;

pub static GRAPH_NODES_PAGE_START_BYTES: usize  = 0 as usize;
pub static GRAPH_NODES_PAGE_END_BYTES: usize  = 0 as usize;

pub static GRAPH_EDGES_PAGE_START_BYTES: usize  = 0 as usize;
pub static GRAPH_EDGES_PAGE_END_BYTES: usize  = 0 as usize;

// These all Need to Be 8 Bytes
pub static SDB_CONFIG_PAGE: &'static str = "[::SDBC]";
pub static GRAPH_NODE_PAGE: &'static str = "[::GNPG]";
pub static GRAPH_EDGE_PAGE: &'static str = "[::GEPG]";

pub static WRITE_COMPLETE: &'static str = "[::::WC]";
pub static PAGE_START: &'static str = "[PGESTR]";
pub static PAGE_END: &'static str = "[PGEEND]";

pub static ROW_PREFIX_GRAPH_REF: &'static str = "[::::GR]";
pub static ROW_PREFIX_NODE_ROW: &'static str = "[::::NR]";
pub static ROW_PREFIX_EDGE_ROW: &'static str = "[::::ER]";

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

/*
#[test]
fn test_common_none () 
{
  assert_eq!( COMMON_NONE.bytes().len(), DGLABEL_BYTES );
}
*/