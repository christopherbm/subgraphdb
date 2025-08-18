use std::fs::{ File };
use std::path::PathBuf;
use crate::datagramv2::internal_grams::{ Label };
use crate::datagramv2::rows::{ affix_to_type, AffixType, PageType };
use crate::utils::open_file;
use crate::executor::core::CoreExecutor;

#[derive( Debug, PartialEq )]
pub enum EmptySpace { HasEnough, NotEnough, ExactlyEnough }

/// Start and End exlucude and include affix respectively
/// [affix]...[affix]
/// start [ ... ] end
#[derive( Debug )]
pub struct PlannerPage 
{
  pub page_type: PageType,
  pub start_pos: u64,
  pub end_pos: Option<u64>,
  pub empty_cell_count: Option<u64>,
  pub empty_cell_start_pos: Option<u64>,
  pub graph_name: Option<String>,
  pub graph_uuid: Option<String>,
}

impl PlannerPage 
{
  pub fn new ( page_type: PageType, start_pos: u64 ) -> PlannerPage 
  {
    PlannerPage 
    { 
      page_type: page_type, 
      start_pos: start_pos, 
      end_pos: None, 
      empty_cell_count: None, 
      empty_cell_start_pos: None,
      graph_name: None,
      graph_uuid: None,
    }
  }
}

#[derive( Debug )]
pub struct WriteNewGraphPlanner<'a>
{
  pub path: String,
  pub graph_name: &'a Label,
  pub current_page_type: Option<PageType>,
  pub db_page: Option<PlannerPage>,
  pub pages: Vec<PlannerPage>,
  pub end_pos: Option<u64>, // end of file affix position 
  pub err_state: Option<String>,
}

impl WriteNewGraphPlanner<'_>
{
  pub fn new ( path: String, graph_name: &Label ) -> WriteNewGraphPlanner
  {
    WriteNewGraphPlanner 
    { 
      path: path,
      graph_name: graph_name,
      current_page_type: None,
      db_page: None,
      pages: Vec::new(),
      end_pos: None,
      err_state: None,
    }
  }

  pub fn plan ( &mut self ) 
  {
    let mut open_res = open_file( &PathBuf::from( &self.path ));
    while self.next( open_res.as_mut().unwrap() ) == true { continue; }
  }

  pub fn next ( &mut self, f: &mut File ) -> bool 
  {
    let affix = CoreExecutor::next_affix( f );
    if affix.is_some() 
    {
      //println!( "{:?}", affix );
      let affix_type: Option<AffixType> = affix_to_type( &affix.unwrap() );
      if affix_type.is_some() 
      {
        match affix_type.unwrap() 
        {
          AffixType::DBPage => 
          { 
            self.process_db_page( f );
            return true;
          }

          AffixType::DataPage => 
          { 
            self.process_data_page( f );
            return true;
          }

          AffixType::BuildId => 
          {
            CoreExecutor::skip_build_id_row( f );
            return true;
          }

          AffixType::DBNickname => 
          {
            CoreExecutor::skip_db_nickname_row( f );
            return true;
          }

          AffixType::Graph => 
          {
            self.process_graph_row( f );
            return true;
          }

          AffixType::StartEmpty =>
          {
            self.process_start_empty( f );
            return true;
          }
          
          AffixType::Placeholder => 
          {
            //self.process_end( f );
            return true;
          }

          AffixType::End => 
          {
            self.process_end( f );
            return false;
          }

          _ => {}
        }
      }
    }
    false
  }
}

// Process Pages
impl WriteNewGraphPlanner<'_>
{
  pub fn process_db_page ( &mut self, f: &mut File ) 
  {
    self.toggle_current_page( AffixType::DBPage );
    if self.db_page.is_none() 
    {
      let pos_res = CoreExecutor::file_position( f );
      if pos_res.is_ok() 
      {
        self.db_page = Some( PlannerPage::new( PageType::DBPage, pos_res.unwrap() - 8 ));
      }
    }
    else 
    {
      let pos_res = CoreExecutor::file_position( f );    
      if pos_res.is_ok() 
      {
        self.db_page.as_mut().unwrap().end_pos = Some( pos_res.unwrap() );
      }
    }
  }

  pub fn process_data_page ( &mut self, f: &mut File ) 
  {
    let pos_res = CoreExecutor::file_position( f );
    if pos_res.is_ok() 
    {
      self.toggle_current_page( AffixType::DataPage );
      if self.pages.len() > 0
      {
        if self.pages.last().unwrap().end_pos.is_some() == false 
        {
          self.pages.last_mut().unwrap().end_pos = Some( pos_res.unwrap() );
          return;
        }
      }
      
      if pos_res.is_ok() 
      {
        self.pages.push( PlannerPage::new( PageType::DataPage, pos_res.unwrap() - 8 ));
      }
    }
  }
}

impl WriteNewGraphPlanner<'_>
{
  pub fn process_start_empty ( &mut self, f: &mut File ) 
  {
    if self.current_page_type.is_some() 
    {
      match self.current_page_type.as_ref().unwrap() 
      {
        PageType::DBPage => 
        { 
          self.process_db_start_empty( f );
          return;
        }

        PageType::DataPage => 
        {
          self.process_data_start_empty( f ); 
          return;
        }

        _ => {}
      }
    }
    self.err_state = Some( String::from( "Error reading stream position (SE1)." ));
  }

  pub fn process_db_start_empty ( &mut self, f: &mut File ) 
  {
    let pos_res = CoreExecutor::file_position( f );    
    if pos_res.is_ok() 
    {
      self.db_page.as_mut().unwrap().empty_cell_start_pos = Some( pos_res.unwrap() - 8 );
      let skip_res = CoreExecutor::skip_empty_cells( f );
      if skip_res.is_ok() 
      { 
        self.db_page.as_mut().unwrap().empty_cell_count = Some( skip_res.unwrap().1 );
        return;
      }
    }
    self.err_state = Some( String::from( "Error reading stream position (SE2)." ));
  }
  
  pub fn process_data_start_empty ( &mut self, f: &mut File ) 
  {
    let pos_res = CoreExecutor::file_position( f );
    if pos_res.is_ok() && self.pages.len() > 0
    {
      self.pages.last_mut().unwrap().empty_cell_start_pos = Some( pos_res.unwrap() - 8 );
      let skip_res = CoreExecutor::skip_empty_cells( f );
      if skip_res.is_ok() 
      {
        self.pages.last_mut().unwrap().empty_cell_count = Some( skip_res.unwrap().1 );
        return;
      }
    }
    self.err_state = Some( String::from( "Error processing DataPage empty cells." ));
  }

  pub fn process_graph_row ( &mut self, f: &mut File ) 
  {
    if self.current_page_type.is_some() 
    {
      match self.current_page_type.as_ref().unwrap() 
      {
        PageType::DBPage => { self.process_db_page_graph_row( f ); },
        PageType::DataPage => { self.process_data_page_graph_row( f ); },
        PageType::AJMPage => { /* no action */ }
      }
      return;
    }
    self.err_state = Some( String::from( "Error processing graph." ));
  }

  pub fn process_db_page_graph_row ( &mut self, f: &mut File ) 
  {
    let graph_row_res = CoreExecutor::read_graph_row( f );
    if graph_row_res.is_ok() 
    {
      if ( graph_row_res.unwrap().1 == self.graph_name.unwrap() ) == true 
      {
        self.err_state = Some( String::from( "Error: Graph exists." ));
      }
      return;
    }
    self.err_state = Some( String::from( "Error processing graph." ));
  }

  pub fn process_data_page_graph_row ( &mut self, f: &mut File ) 
  {
    if self.pages.len() > 0
    {
      let graph_row_res = CoreExecutor::read_graph_row( f );
      if graph_row_res.is_ok() 
      {
        if self.pages.last().unwrap().graph_uuid.is_some() == false 
        {
          self.pages.last_mut().unwrap().graph_uuid = Some( graph_row_res.as_ref().unwrap().0.clone() );
          self.pages.last_mut().unwrap().graph_name = Some( graph_row_res.unwrap().1 );
          return;
        }
      }
    }
    self.err_state = Some( String::from( "Error processing DataPage GraphRow." ));
  }

  pub fn process_end ( &mut self, f: &mut File ) 
  {
    let pos_res = CoreExecutor::file_position( f );
    if pos_res.is_ok() 
    {
      self.end_pos = Some( pos_res.unwrap() );
      return;
    }
    self.err_state = Some( String::from( "Error reading stream position (EOF)." ));
  }
}

impl WriteNewGraphPlanner<'_>
{
  pub fn toggle_current_page ( &mut self, affix: AffixType ) 
  {
    match affix 
    {
      AffixType::DBPage =>
      {
        if self.current_page_type.is_none() { self.current_page_type = Some( PageType::DBPage ); }
        else { self.current_page_type = None; }
      }

      AffixType::DataPage =>
      {
        if self.current_page_type.is_none() { self.current_page_type = Some( PageType::DataPage ); }
        else { self.current_page_type = None; }
      }

      _ => {}   
    }
  }

  pub fn db_page_has_space ( &self, space: usize ) -> EmptySpace
  {
    if self.db_page.is_some() 
    {
      if self.db_page.as_ref().unwrap().empty_cell_count.is_some() 
      {
        if space < self.db_page.as_ref().unwrap().empty_cell_count.unwrap() as usize 
        { 
          return EmptySpace::HasEnough; 
        }
        
        if space == self.db_page.as_ref().unwrap().empty_cell_count.unwrap() as usize 
        { 
          return EmptySpace::ExactlyEnough; 
        }
      }
    }
    EmptySpace::NotEnough
  }

  pub fn has_data_pages ( &self ) -> bool { self.pages.len() > 0 }

  pub fn fetch_data_pages_by_graph ( &self, name: &str ) -> Vec<&PlannerPage>
  {
    let mut ret: Vec<&PlannerPage> = Vec::new();
    for page in self.pages.iter() 
    {
      if page.graph_name.is_some() 
      {
        if page.graph_name.as_ref().unwrap() == name { ret.push( page ); }
      }
    }
    ret
  }
}

mod tests 
{
  use super::*;
  use std::fs::metadata;

  const PAGE_SIZE: usize = 4096;

  #[test]
  fn test_planner_1 () 
  {
    let path_str = "../test_data/new.sdb";
    let name_label = Label::new( String::from( "devs" ) );
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), name_label.as_ref().unwrap() );
    planner.plan();

    assert_eq!( planner.db_page.is_some(), true );
    assert_eq!( planner.db_page.as_ref().unwrap().start_pos, 0 );
    assert_eq!( planner.db_page.as_ref().unwrap().end_pos, Some( 4088 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_count, Some( 476 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_start_pos, Some( 264 ));
    
    assert_eq!( planner.end_pos, Some( 4096 ));
    assert_eq!( planner.err_state, None );
  }

  #[test]
  fn test_planner_2 () 
  {
    let path_str = "../test_data/new_with_data_page.sdb";
    let name_label = Label::new( String::from( "devs" ) );
    let mut planner = WriteNewGraphPlanner::new( path_str.to_string(), name_label.as_ref().unwrap() );
    planner.plan();

    assert_eq!( planner.db_page.is_some(), true );
    assert_eq!( planner.db_page.as_ref().unwrap().start_pos, 0 );
    assert_eq!( planner.db_page.as_ref().unwrap().end_pos, Some( 4088 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_count, Some( 476 ));
    assert_eq!( planner.db_page.as_ref().unwrap().empty_cell_start_pos, Some( 264 ));
    
    assert_eq!( planner.pages.len(), 1 );
    assert_eq!( planner.pages.get(0).unwrap().start_pos, 4096);
    assert_eq!( planner.pages.get(0).unwrap().end_pos, Some( 8184 ) );
    assert_eq!( planner.pages.get(0).unwrap().empty_cell_count, Some( 493 ));
    assert_eq!( planner.pages.get(0).unwrap().empty_cell_start_pos, Some( 4224 ));
    assert_eq!( planner.pages.get(0).unwrap().graph_name.is_some(), true );
    assert_eq!( planner.pages.get(0).unwrap().graph_uuid.is_some(), true );

    assert_eq!( planner.end_pos, Some( 8192 ));
    assert_eq!( planner.err_state, None );
    assert_eq!( metadata( &PathBuf::from( path_str ) ).unwrap().len(), (PAGE_SIZE * 2) as u64 );
  }
}