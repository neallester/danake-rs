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
use std::cell::RefCell;
use std::cell::RefMut;


pub trait EntityStructClone {

    fn entity_clone (&self) -> Self where Self: Sized;

}

pub trait EntityStructEq {

    fn entity_eq (&self, other: &Self) -> bool where Self: Sized ;
}

pub trait EntityStructSerialize {

    fn entity_serialize<'a> (&self, serializer: Box<Serializer + 'a>) -> Result<(), erased_serde::Error>;

    fn to_json(&self) -> Result<std::string::String, serde_json::Error>;
}

impl<C: Clone> EntityStructClone for C {

    fn entity_clone (&self) -> C { return self.clone() }

}

impl<E: Eq> EntityStructEq for E {

    fn entity_eq (&self, other: &Self) -> bool{
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

pub trait EntityStruct<'a>: Send + Sync + EntityStructClone + EntityStructEq + EntityStructSerialize {

    fn as_entity_struct (&'a self) -> &'a EntityStruct;

}
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


impl <'a, S: Sized + Send + Sync + Eq + Clone + serde::Serialize + EntityStructClone + EntityStructEq + EntityStructSerialize>EntityStruct<'a> for S {

    fn as_entity_struct (&'a self) -> &'a EntityStruct {
        return self as &'a EntityStruct;
    }

}

#[derive(Debug)]
pub struct Entity<'a, E: ?Sized> where E: 'a + EntityStruct<'a> {

    item: Option<Box<RefCell<E>>>,
    id: Uuid,
    base_version: u32,
    return_channel: Option<Sender<Entity<'a, EntityStruct<'a>>>>,
    // TODO return channel needs to work on an EntityStruct, not parameter E so that we can have
    //      one receiver for all entity types
    // TODO id should be a reference instead of the struct, with the owner being the batch object.

}

impl<'a, E> Drop for Entity<'a, E> where E: 'a + ?Sized + EntityStruct<'a> {
    fn drop(&mut self) {
        let sender = self.return_channel.take();
        match sender {
            Some (sender) => {
                let item = self.item.take();
                let return_entity = Entity {
                    item: item,
                    id: self.id.clone(),
                    base_version: self.base_version.clone(),
                    return_channel: None,
                };
                println! ("Sending return");
                let _ = sender.send(return_entity);
            },
            None => {}
        }


    }
}

// Example of the compilation problem I am having
//use std::sync::mpsc::Sender;
//
//pub trait MyTrait{}
//
//pub struct MyContainer<G> where G: 'static + MyTrait {
//
//    item: Option<Box<G>>,
//    sender: Sender<Option<Box<MyTrait>>>,
//
//}
//
//impl <G> MyContainer<G> where G: MyTrait {
//
//    fn send (&mut self) {
//        let item = self.item.take();
//        self.sender.send(item);
//    }
//}
//
//fn main(){}

//impl <G> MyContainer<G> where G: MyTrait + 'static {
//
//    fn send (&mut self) {
//        let item = self.item.take();
//        self.sender.send(item.map(|item| item as _));
//    }
//}


//error[E0308]: mismatched types
//--> src/main.rs:16:26
//|
//16 |         self.sender.send(item);
//|                          ^^^^ expected trait MyTrait, found type parameter
//|
//= note: expected type `std::option::Option<std::boxed::Box<MyTrait + 'static>>`
//found type `std::option::Option<std::boxed::Box<G>>`
//= help: here are some functions which might fulfill your needs:
//- .and(...)
//- .and_then(...)
//- .map(...)
//- .map_or(...)
//- .map_or_else(...)

//Working Code
//use std::sync::mpsc::Sender;
//
//pub trait MyTrait{}
//
//pub struct MyContainer<G> where G: 'static + MyTrait {
//
//    item: Option<Box<G>>,
//    sender: Sender<Option<Box<MyTrait>>>,
//
//}
//
//impl <G> MyContainer<G> where G: MyTrait {
//
//    fn send (&mut self) {
//        let item = self.item.take();
//        self.sender.send(item.map(|item| item as _));
//    }
//}
//
//fn main(){}

// POSSIBLE SOLUTION
// Use a helper function EntityStruct.asTrait(&self) -> EntityStruct
// Which can be provided a default implementation for all Sized

impl<'a, E> Entity<'a, E> where E: ?Sized + EntityStruct<'a> {

    fn borrow_mut (&mut self) ->  RefMut<E> {
        match self.item {
            Some (ref mut item) => {
                let item = &**item;
                let item = item.borrow_mut();
                return item;
            }
            None => panic!("danake::Entity without Item"),
        }
    }

    fn item_json(&self) -> Result<std::string::String, serde_json::Error> {
        match self.item {
            Some (ref item) => {
                let item = &**item;
                let item = item.borrow_mut();
                item.to_json()
            }
            None => Ok (String::from("{}")),
        }
    }
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
    use uuid::Uuid;
    use std::cell::RefCell;
    use std::thread;



    #[derive(Clone, PartialEq, Eq, Debug, Serialize)]
    struct TestStruct {
        a: i32,
        v: Vec<i32>,
    }


    fn mock_test_entity<'a> (uuid: Uuid, sender: Sender<Entity<'a, EntityStruct>>) -> Entity<'a, TestStruct> {
        let s1 = TestStruct {
            a: 100,
            v: vec![200, 300],
        };
        Entity {
            item: Some(Box::new(RefCell::new(s1))),
            id: uuid.clone(),
            base_version: 0,
            return_channel: Some(sender),
        }
    }

    #[test]
    fn entity_test() {
        let uuid = Uuid::new_v4();
        let uuid_thread = uuid.clone();
        let (tx, rx): (Sender<Entity<EntityStruct>>, Receiver<Entity<EntityStruct>>) = channel();
        thread::spawn (move || {
            let mut my_entity = mock_test_entity(uuid_thread, tx);
            {
                let item1 = my_entity.borrow_mut();
                assert_eq!(100, item1.a);
            }
            assert_eq!(100, my_entity.borrow_mut().a);
            let mut item = my_entity.borrow_mut();
            assert_eq!(100, item.a);
            item.a = 200;
            assert_eq!(200, item.a);
        });
        let returned_entity = rx.recv();
        assert_eq!("{\"a\":200,\"v\":[200,300]}", returned_entity.unwrap().item_json().unwrap());
    }

    #[test]
    fn entity_struct_test () {
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
