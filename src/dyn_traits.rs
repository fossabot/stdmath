use std::{
    any::Any,
    cmp::{Eq, Ordering, PartialEq},
    fmt::{self, Debug},
    hash::{Hash, Hasher},
};

trait Value {
    fn as_any(&self) -> &dyn Any;
    fn _cmp(&self, other: &dyn Value) -> Option<Ordering>;
    fn _clone(&self) -> Box<dyn Value> {
        unimplemented!()
    }
    fn _debug(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
    fn _hash(&self, _state: &mut dyn Hasher) {
        unimplemented!()
    }
}

impl PartialEq<dyn Value> for dyn Value {
    fn eq(&self, other: &dyn Value) -> bool {
        if let Some(Ordering::Equal) = self._cmp(other) {
            return true;
        }
        false
    }
}

impl Eq for dyn Value {}

impl PartialOrd<dyn Value> for dyn Value {
    fn partial_cmp(&self, other: &dyn Value) -> Option<Ordering> {
        self._cmp(other)
    }
}

impl Debug for dyn Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self._debug(f)
    }
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Self {
        self._clone()
    }
}

impl Hash for Box<dyn Value> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self._hash(state);
    }
}

macro_rules! stage_default_methods {
    () => {};
    (ALL) => {
        stage_default_methods!(as_any _cmp _debug _clone _hash);
    };
    (as_any $($rest:tt)*) => {
        fn as_any(&self) -> &dyn Any {
            self
        }
        stage_default_methods!($($rest)*);
    };
    (_cmp $($rest:tt)*) => {
        fn _cmp(&self, other: &dyn Value) -> Option<Ordering> {
            other
                .as_any()
                .downcast_ref::<Self>()
                .map_or(None, |other| PartialOrd::partial_cmp(self, other))
        }
        stage_default_methods!($($rest)*);
    };
    (_debug $($rest:tt)*) => {
        fn _debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            Debug::fmt(self, f)
        }
        stage_default_methods!($($rest)*);
    };
    (_clone $($rest:tt)*) => {
        fn _clone(&self) -> Box<dyn Value> {
            Box::new(self.clone()) as Box<dyn Value>
        }
        stage_default_methods!($($rest)*);
    };
    (_hash $($rest:tt)*) => {
        fn _hash(&self, mut state: &mut dyn Hasher) {
            self.hash(&mut state)
        }
        stage_default_methods!($($rest)*);
    };
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
struct A;
impl Value for A {
    stage_default_methods!(ALL);
}
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug, Hash)]
struct B;
impl Value for B {
    stage_default_methods!(ALL);
}

#[derive(Clone, Debug, Eq, PartialOrd, Hash)]
struct C {
    a: Box<dyn Value>,
    b: Box<dyn Value>,
}

impl PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        (&self.a, &self.b) == (&other.a, &other.b)
    }
}

impl Value for C {
    stage_default_methods!(ALL);
}

pub fn main() {
    let vec1: Vec<Box<dyn Value>> = vec![Box::new(A), Box::new(B)];
    let vec2: Vec<Box<dyn Value>> = vec![Box::new(A), Box::new(B)];
    let res = vec1 == vec2;
    println!("{}", res);

    println!(
        "{}",
        Box::new(A) as Box<dyn Value> == Box::new(B) as Box<dyn Value>
    );

    let a = A;
    let b = B;
    println!("{:?}", a);
    println!("{:?}", a.clone());
    println!("{:?}", b);
    println!("{:?}", b.clone());

    let c = Box::new(a) as Box<dyn Value>;
    println!("{:?}", c);
    println!("{:?}", c.clone());

    let val1 = C {
        a: Box::new(A),
        b: Box::new(B),
    };
    let val2 = C {
        a: Box::new(A),
        b: Box::new(B),
    };

    println!("{:?}", val1);
    println!("{:?}", val2);
    println!("{:?}", val1 == val2);

    let val3 = Box::new(val1) as Box<dyn Value>;
    println!("{:?}", val3);

    let mut state = std::collections::hash_map::DefaultHasher::new();
    val2.hash(&mut state);
    println!("{:x}", state.finish());

    use super::exclude::ExcludedIteratorExt;

    let val1 = vec![
        Box::new(A) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
    ];
    let val2 = vec![
        Box::new(B) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
        Box::new(A) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
        Box::new(B) as Box<dyn Value>,
    ];
    let iter1 = val1.into_iter();
    let iter2 = val2.into_iter();

    let res = iter1.exclude(iter2).include_overflow();
    println!("{:?}", res.collect::<Vec<_>>());
}
