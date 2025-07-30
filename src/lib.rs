pub mod common;
pub mod cli;
pub mod utils;
pub mod tokenize;
pub mod sdb_config;
pub mod sdb;
pub mod cmd;
pub mod graph;
pub mod datagramv2;
pub mod executor;
pub mod parser;
pub mod planner;

use std::fs;
use std::path::PathBuf;
//use sdb::single_file::{ init_sf_db };
use cli::{ ArgOneAction, ArgTwoAction, ArgThreeAction, check_first_arg, check_second_arg, check_third_arg };
//use fdm::{ path_exists };

/*
  cargo run -- /platonic3d/subgraphdbv2/test_data/sf/test1.sdb
  cargo run -- /platonic3d/subgraphdbv2/test_data/sf/new_db.sdb -sf
  cargo run -- /platonic3d/subgraphdbv2/test_data/sf/test_db1.sdb -sf mytestdb
*/

static VERSION: &'static str = "0.1.0-01faaa5ed951";
static BUILD_UUID: &'static str = "7a402309-36a0-4120-a23b-01faaa5ed951";

// this will become cli/process_args( args: Args )
fn main() 
{
  let mut input_path_exists: bool = false;
  let mut create_mode: bool = false;
  let mut init_sf: bool = false;
  let mut init_mf: bool = false;
  let mut nickname: Option<String> = None;

  match check_first_arg( std::env::args().nth( 1 ))
  {
    ArgOneAction::ArgNotFound => println!( "arg error" ),
    ArgOneAction::ShowFullHelp => println!( "show full help" ),
    ArgOneAction::InteractiveMode => println!( "interactive mode" ),
    ArgOneAction::CheckFilePath => 
    {
      //let file_exists = path_exists( PathBuf::from( std::env::args().nth( 1 ).unwrap() ));
      //if file_exists.is_ok() { input_path_exists = file_exists.unwrap(); }
    },
    ArgOneAction::CheckDirPath => 
    {
      //let dir_exists = path_exists( PathBuf::from( std::env::args().nth( 1 ).unwrap() ));
      //if dir_exists.is_ok() { input_path_exists = dir_exists.unwrap(); }
    }
    ArgOneAction::CreateNewFile => create_mode = true,
  }

  match check_second_arg( std::env::args().nth( 2 )) 
  {
    ArgTwoAction::ArgNotFound => println!( "arg error" ),
    ArgTwoAction::CreateSingleFile => init_sf = true,
    ArgTwoAction::CreateMultiFile => init_mf = true,
  }
  
  match check_third_arg( std::env::args().nth( 3 )) 
  {
    ArgThreeAction::ArgNotFound => println!( "no third arg" ),
    ArgThreeAction::IsNickname => nickname = std::env::args().nth( 3 ),
  }

  if create_mode && !input_path_exists
  {
    if init_sf 
    {
      //init_sf_db( BUILD_UUID, PathBuf::from( std::env::args().nth( 1 ).unwrap() ), nickname );
    }
  }

  println!( "{:?}", std::env::args() )
}