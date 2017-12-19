mod storage;

extern crate serde;
extern crate uuid;
use self::uuid::Uuid;
use self::serde::Serialize;
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

impl<C: Clone> EntityStructClone for C {

    fn entity_clone (&self) -> C { return self.clone() }

}

impl<E: Eq> EntityStructEq for E {

    fn entity_eq (&self, other: &Self) -> bool {
        return self.eq(other)
    }
}

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



pub struct Entity<'a, E: ?Sized> where E: EntityStructClone {

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
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::Receiver;
    use serde::Serialize;
    use serde::de::DeserializeOwned;

    #[derive(Clone, PartialEq, Eq)]
    pub struct MyStruct {
        a: i32,
        v: Vec<i32>,
    }

    impl EntityStruct for MyStruct {   }



    #[test]
    fn it_works() {
        let (tx, rx): (Sender<Entity<EntityStruct>>, Receiver<Entity<EntityStruct>>) = channel();
        let v: Vec<Box<Entity<EntityStruct>>> =  Vec::new();

        assert_eq!(2 + 2, 4);
    }
}
