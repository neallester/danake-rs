mod storage;

extern crate serde;
extern crate uuid;
use self::uuid::Uuid;
use self::serde::{Serialize, Serializer};
use self::serde::de::DeserializeOwned;
use std::sync::mpsc::Sender;

struct Batch {

}

struct Token {}

pub trait EntityStructClone {

    fn entity_clone (&self) -> Self where Self: Sized;

}

pub trait EntityStructEq {

    fn entity_eq (&self, other: &Self) -> bool where Self: Sized;
}

//pub trait EntityStructSerialize {
//    fn serialize<S>(&self, serializer: Box<S>) -> Result<S::Ok, S::Error>
//        where
//            S: Serializer;
//}

impl<C: Clone> EntityStructClone for C {

    fn entity_clone (&self) -> C { return self.clone() }

}

impl<E: Eq> EntityStructEq for E {

    fn entity_eq (&self, other: &Self) -> bool {
        return self.eq(other)
    }

}

//impl<S: Serialize> EntityStructSerialize for S {
//    fn serialize<S>(&self, serializer: Box<S>) -> Result<S::Ok, S::Error> {
//        return self.serialize(serializer);
//    }
//
//}

pub trait EntityStruct: EntityStructClone + EntityStructEq {}
// This indirection ^ prevents an implementation conflict for descendants
//                  |
//error[E0119]: conflicting implementations of trait `EntityStruct` for type `MyStruct`:
//--> src/lib.rs:49:1
//|
//37 | / impl<T: Clone> EntityStruct for T {
//    38 | |
//    39 | |     fn clone_entity (&self) -> T {
//        40 | |         return self.clone()
//        41 | |     }
//    42 | | }
//| |_- first implementation here
//...
//49 |   impl EntityStruct for MyStruct {}
//|   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `MyStruct`


pub struct Entity<'a, E: ?Sized> where E: EntityStruct {

    item: Box<E>,
    id: Uuid,
    base_version: u32,
    token: Option<&'a Token>,
    return_channel: Sender<Entity<'a, E>>,

}



#[cfg(test)]
mod tests {

    use Entity;
    use EntityStruct;
    use EntityStructClone;
    use EntityStructEq;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::Receiver;
    use serde::Serialize;
    use serde::de::DeserializeOwned;

    #[derive(Clone, PartialEq, Eq, Debug)]
    struct TestStruct {
        a: i32,
        v: Vec<i32>,
    }

    impl EntityStruct for TestStruct {   }



    #[test]
    fn it_works() {
        let (tx, rx): (Sender<Entity<EntityStruct>>, Receiver<Entity<EntityStruct>>) = channel();
        let v: Vec<Box<Entity<EntityStruct>>> =  Vec::new();

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn clone_eq_tests() {
        let s1 = TestStruct {
            a: 100,
            v: vec![200, 300],
        };
        let s2 = s1.entity_clone();
        assert_eq!(s1, s2);
        assert! (s1.entity_eq (&s2));
        assert_eq!(100, s1.a);
        assert_eq!(2, s1.v.len());
        assert_eq!(&200, s1.v.get(0).unwrap());
        assert_eq!(&300, s1.v.get(1).unwrap());
        assert_eq!(100, s2.a);
        assert_eq!(2, s2.v.len());
        assert_eq!(&200, s2.v.get(0).unwrap());
        assert_eq!(&300, s2.v.get(1).unwrap());
        let s3 = TestStruct {
            a: 200,
            v: vec![200, 300],
        };
        assert_ne! (s1, s3);
        assert! (!s1.entity_eq (&s3));
        let s4 = TestStruct {
            a: 100,
            v: vec![500, 300],
        };
        assert_ne! (s1, s4);
        assert! (!s1.entity_eq (&s4));
    }
}
