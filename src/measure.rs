use std::iter;
use std::ops::Add;

/// The class of monoids
///
/// The class of monoids (types with an associative binary operation that has
/// an identity).
///
/// An instance _M_ should satisfy the following laws:
///
///  + _x_`.join(`_M_`::unit())` = _x_
///  + _M_`::unit().join(`_x_`)` = _x_
///  + _x_`.join(`_y_`.join(`_z_`))` = _z_`.join(`_x_`.join(`_y_`))`
///  + _M_`::concat(`_a_`)` = _a_`.fold(`_M_`::unit,`_M_`::join)`
///
pub trait Monoid {
    fn unit() -> Self;
    fn join(self, other: Self) -> Self;

    #[inline]
    fn concat<F>(xs: F) -> Self
    where F: Iterator<Item=Self>
        , Self: Sized
    {
        xs.fold(Self::unit(), Self::join)
    }
}

pub trait Measurable<V>
where V: Monoid {
    fn measure(&self) -> V;
}

#[derive(Clone, Copy)]
pub struct Length(usize);
impl Monoid for Length {
    #[inline] fn unit() -> Self { Length(0) }
    #[inline] fn join(self, other: Self) -> Self { Length(self.0 + other.0) }
}

impl<'a, A: 'a> Monoid for Box<Iterator<Item=A> + 'a> {
    #[inline] fn unit() -> Self { box iter::empty() }
    #[inline] fn join(self, other: Self) -> Self { box self.chain(other) }
}

impl<A> Monoid for Vec<A> {
    #[inline] fn unit() -> Self { vec![] }
    #[inline] fn join(self, other: Self) -> Self {
        let mut s = self; let mut o = other;
         s.append(&mut o);
         s
     }
}

impl<'a, V, A, I> Measurable<V> for &'a I
where V: Monoid
    , A: Measurable<V> + 'a
    , &'a I: IntoIterator<Item=&'a A>
    {
        #[inline]
        fn measure(&self) -> V {
            V::concat(self.into_iter().map(A::measure))
        }
    }

// impl<'a, V, A, I> Measurable<V> for I
// where V: Monoid
//     , A: Measurable<V> + 'a
//     , I: Iterator<Item=&'a A>
//     {
//         #[inline]
//         fn measure(&self) -> V {
//             V::concat(self.map(A::measure))
//         }
//     }
