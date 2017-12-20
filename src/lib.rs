mod storage;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate uuid;
extern crate erased_serde;
use self::uuid::Uuid;
use erased_serde::{Serialize, Serializer};
use std::sync::mpsc::Sender;
use std::result::Result;

struct Batch {

}

struct Token {}

pub trait EntityStructClone {

    fn entity_clone (&self) -> Self where Self: Sized;

}

pub trait EntityStructEq {

    fn entity_eq (&self, other: &Self) -> bool where Self: Sized;
}

pub trait EntityStructSerialize {

    fn entity_serialize<'a> (&self, mut serializer: Box<Serializer + 'a>) -> Result<(), erased_serde::Error>;

    fn to_json(&self) -> Result<std::string::String, serde_json::Error>;
}

impl<C: Clone> EntityStructClone for C {

    fn entity_clone (&self) -> C { return self.clone() }

}

impl<E: Eq> EntityStructEq for E {

    fn entity_eq (&self, other: &Self) -> bool {
        return self.eq(other)
    }

}

impl<S: serde::Serialize> EntityStructSerialize for S
{
    fn entity_serialize<'a> (&self, mut serializer: Box<Serializer + 'a>) -> Result<(), erased_serde::Error> {
        let result = self.erased_serialize(&mut *serializer);
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err (err),
        }
    }

    fn to_json(&self) -> Result<std::string::String, serde_json::Error> {
        serde_json::to_string (self)
    }

}

pub trait EntityStruct: EntityStructClone + EntityStructEq + EntityStructSerialize {}
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

    extern crate serde_json;

    use Entity;
    use EntityStruct;
    use EntityStructClone;
    use EntityStructEq;
    use EntityStructSerialize;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::Receiver;
    use erased_serde::{Serializer};
    use std::rc::Rc;


    #[derive(Clone, PartialEq, Eq, Debug, Serialize)]
    struct TestStruct {
        a: i32,
        v: Vec<i32>,
    }

    impl <'a> EntityStruct for TestStruct {   }



    #[test]
    fn it_works() {
        let (tx, rx): (Sender<Entity<EntityStruct>>, Receiver<Entity<EntityStruct>>) = channel();
        let v: Vec<Box<Entity<EntityStruct>>> =  Vec::new();

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn clone_entity_struct () {
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

        assert_eq! ("{\"a\":100,\"v\":[200,300]}", s1.to_json().unwrap());

        let bytes = Vec::new();
        let mut json = Rc::new (self::serde_json::Serializer::new(bytes));
        let json_ref = Rc::get_mut(&mut json).unwrap();
        let erased: Box<Serializer> = Box::new(Serializer::erase(json_ref));
        let _ = s1.entity_serialize(erased);
    }
}
