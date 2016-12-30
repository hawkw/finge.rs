//! # finge.rs
//!
//! [2-3 finger trees](http://www.staff.city.ac.uk/~ross/papers/FingerTree.html)
//! are a functional representation of persistent sequences
//! supporting access to the ends in amortized constant time, and concatenation
//! and splitting in time logarithmic in the size of the smaller piece.
//! Representations achieving these bounds have appeared previously, but 2-3
//! finger trees are much simpler, as are the operations on them. Further, by
//! defining the split operation in a general form, we obtain a general purpose
//! data structure that can serve as a sequence, priority queue, search tree,
//! priority search queue and more.

#![feature(const_fn)]
#![feature(box_syntax)]
#![feature(slice_patterns)]
use std::rc::Rc;

pub mod measure;
pub use self::measure::Measurable;
use measure::Monoid;

pub trait FingerTreeLike<A>: Sized {
    fn is_empty() -> bool;

    fn push(&self, A) -> Self;
    fn pop(&self) -> (Self, Option<A>);

    fn append<T>(&self, other: T) -> Self where T: FingerTreeLike<A>;
    fn prepend<T>(&self, other: T) -> Self where T: FingerTreeLike<A>;
}

#[derive(Debug, Clone)]
pub enum FingerTree<A, M>
    { Nil
    , Single(Node<A, M>)
    , Deep { measure: M
           , prefix: Digit<A, M>
           , tree: Rc<FingerTree<A, M>>
           , suffix: Digit<A, M>
           }
    }
#[derive(Debug, Clone)]
pub enum Node<A, M>
    { Leaf(A)
    , Node2(M, Rc<Node<A, M>>, Rc<Node<A, M>>)
    , Node3(M, Rc<Node<A, M>>, Rc<Node<A, M>>, Rc<Node<A, M>>)
    }

impl<A, M> Measurable<M> for Node<A, M>
where A: Measurable<M>
    , M: Monoid
    , M: Clone
    {
        fn measure(&self) -> M {
            match *self { Leaf(ref x) => x.measure()
                        , Node2(ref m, _, _,) => m.clone()
                        , Node3(ref m, _, _, _) => m.clone()

            }
        }
    }

#[derive(Debug, Clone)]
pub struct Digit<A, M>(Vec<Node<A, M>>);

impl<A, M> Measurable<M> for Digit<A, M>
where A: Measurable<M>
    , M: Monoid
    , M: Clone
    {
        fn measure(&self) -> M {
            debug_assert!(self.0.len() > 0);
            debug_assert!(self.0.len() <= 3);
            (&self.0).measure()
        }
    }

use self::FingerTree::*;
use self::Node::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
