pub mod core;
pub mod reader;
pub mod writer;
pub mod core_planner;

/*
// !! page size needs to be configurable, but row size will be fixed. Page size cannot be changed after db creation.
// !! there can be more than 1 sdb config page depending on number and complexity of graphs
// !! knows that "name" is first of indexed values - this would need to be part of graph config as a contraint
// !! any modification to the graph config or the db config will result in a rewrite of the binary data
// let page_size: usize = 4096;

- When a read begins, it chooses a particular transaction id to be its "end mark". This way reads can happen along with
  writes but the reads will have "point-in-time" consistency.
*/

pub struct Executor
{
}
impl Executor
{
  pub fn new () -> Executor
  {
    Executor {}
  } 
}