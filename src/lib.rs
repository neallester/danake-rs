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

// type EntityStruct = Clone + Eq + Serialize + DeserializeOwned;

//pub trait EntityStruct: Clone + Eq + Serialize + DeserializeOwned {}
//
//pub struct Entity<'a, E> where E: Sized + Clone + Eq + Serialize + DeserializeOwned {
//
//    item: E,
//    id: Uuid,
//    base_version: u32,
//    toekn: Option<&'a Token>,
//    return_channel: Sender<Entity<'a, E>>,
//
//}


pub trait EntityStruct: {

    fn clone_entity (&self) -> Self where Self: Sized;

}

impl<T: Clone> EntityStruct for T {

    fn clone_entity (&self) -> Self {
        return self.clone()
    }
}

pub trait ES: EntityStruct {}

#[derive(Clone)]
pub struct MyStruct {}

impl ES for MyStruct {}



pub struct Entity<'a, E> where E: EntityStruct {

    item: E,
    id: Uuid,
    base_version: u32,
    toekn: Option<&'a Token>,
    return_channel: Sender<Entity<'a, E>>,

}



#[cfg(test)]
mod tests {

    use Entity;
    use EntityStruct;
    use ES;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::Sender;
    use std::sync::mpsc::Receiver;
    use serde::Serialize;
    use serde::de::DeserializeOwned;



    #[test]
    fn it_works() {
//        let (tx, rx): (Sender<Box<Entity<ES>>>, Receiver<Box<Entity<ES>>>) = channel();



        assert_eq!(2 + 2, 4);
    }
}
