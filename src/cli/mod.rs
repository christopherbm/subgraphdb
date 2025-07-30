use std::path::PathBuf;
//use fdm::{ is_file, is_dir, has_file_extension };

static ARG_NOT_FOUND: &'static str = "::arg_not_found";
static SHOW_FULL_HELP: &'static str = "::show_full_help";
static CHECK_PATH: &'static str = "::check_path";

/*
  - / Initialize (Create) In-Memory Database (Default Configuration)
  - / Initialize (Create) Database (Default Configuration)
  - / Initialize (Load) Database
  - / Initialize (Create) Database (Default Configuration)
  - / Initialize (Load) Database
  - / List Graphs (IM/SF/MF)
  - / Create New Graph (IM/SF/MF)
  - / Load Graph (SF/MF)
*/

/*
match var 
{
  ArgOneAction::ArgNotFound => (...),
  ArgOneAction::ShowFullHelp => (...),
  ArgOneAction::InteractiveMode => (...),
  ArgOneAction::CheckFilePath => (...),
  ArgOneAction::CheckDirPath => (...),
  ArgOneAction::CreateNewFile => (...),
}
*/
/// 1st CLI Argument Action
#[ derive( Debug, PartialEq )]
pub enum ArgOneAction
{
  ArgNotFound,
  ShowFullHelp,
  InteractiveMode,
  CheckFilePath,
  CheckDirPath,
  CreateNewFile,
}

/*
match var 
{
  ArgTwoAction::ArgNotFound => (...),
  ArgTwoAction::CreateSingleFile => (...),
  ArgTwoAction::CreateMultiFile => (...),
}
*/
/// 2nd CLI Argument Action
#[ derive( Debug, PartialEq )]
pub enum ArgTwoAction
{
  ArgNotFound,
  CreateSingleFile,
  CreateMultiFile,
}

/*
match var 
{
  ArgThreeAction::ArgNotFound => (...),
  ArgThreeAction::IsNickname => (...),
}
*/
/// 3rd CLI Argument Action
#[ derive( Debug, PartialEq )]
pub enum ArgThreeAction
{
  ArgNotFound,
  IsNickname,
}

/// Check First CLI Argument
pub fn check_first_arg ( arg_opt: Option<String> ) -> ArgOneAction 
{
  if arg_opt.is_some() 
  {
    let arg: String = arg_opt.unwrap();
    if arg == String::from( "-help" ) { return ArgOneAction::ShowFullHelp }
    if arg == String::from( "-im" ) { return ArgOneAction::InteractiveMode }
    
    let path = PathBuf::from( arg );
    
    // existing file or path
    //if is_file( &path ) { return ArgOneAction::CheckFilePath }
    //if is_dir( &path ) { return ArgOneAction::CheckDirPath }
    //if has_file_extension( &path ) { return ArgOneAction::CreateNewFile }
  }
  ArgOneAction::ArgNotFound
}

/// Check Second CLI Argument
pub fn check_second_arg ( arg_opt: Option<String> ) -> ArgTwoAction
{
  if arg_opt.is_some() 
  {
    let arg: String = arg_opt.unwrap();
    if arg == String::from( "-sf" ) { return ArgTwoAction::CreateSingleFile }
    if arg == String::from( "-mf" ) { return ArgTwoAction::CreateMultiFile }
  }
  ArgTwoAction::ArgNotFound
}

/// Check Third Argument
pub fn check_third_arg ( arg_opt: Option<String> ) -> ArgThreeAction 
{
  if arg_opt.is_some() 
  {
    let arg: String = arg_opt.unwrap();
    let first_char_opt = arg.chars().nth( 0 );
    if first_char_opt.is_some()
    {
      let first_char = first_char_opt.unwrap();
      if String::from( first_char ) == "-" 
      {
        // !!! return something
      }
      return ArgThreeAction::IsNickname
    }
  }
  ArgThreeAction::ArgNotFound
}

// ---------------------------------------------------------------------------------------------------------------------

/// Show Full Help
fn show_full_help () 
{
  println!( "Full help" )
}

// ---------------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests 
{
  use super::*;
  
  #[test]
  fn test_check_first_arg () 
  {
    assert_eq!( check_first_arg( None ), ArgOneAction::ArgNotFound );
    assert_eq!( check_first_arg( Some( String::from( "-help" ))), ArgOneAction::ShowFullHelp );
    assert_eq!( check_first_arg( Some( String::from( "-im" ))), ArgOneAction::InteractiveMode );
    assert_eq!( 
      check_first_arg( 
        Some( String::from( "/platonic3d/subgraphdbv2/test_data/sf/test1.sdb" ))), 
        ArgOneAction::CheckFilePath );
    assert_eq!( 
      check_first_arg( 
        Some( String::from( "/platonic3d/subgraphdbv2/test_data/" ))), 
        ArgOneAction::CheckDirPath );
  }

  #[test]
  fn test_check_second_arg () 
  {
    assert_eq!( check_second_arg( None ), ArgTwoAction::ArgNotFound );
    assert_eq!( check_second_arg( Some( String::from( "-help" ))), ArgTwoAction::ArgNotFound );
    assert_eq!( check_second_arg( Some( String::from( "-sf" ))), ArgTwoAction::CreateSingleFile );
    assert_eq!( check_second_arg( Some( String::from( "-mf" ))), ArgTwoAction::CreateMultiFile );
  }
}