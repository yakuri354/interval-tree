#![no_std]

extern crate alloc;

mod node;
pub mod tree;
mod iterators;
pub use tree::IntervalTree;
pub use iterators::RangePairIter;
