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

// type EntityStructImp = Clone + Eq + Serialize + DeserializeOwned;

//pub trait EntityStructImp: Clone + Eq + Serialize + DeserializeOwned {}
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


pub trait EntityStructImp {

    fn clone_entity (&self) -> Self where Self: Sized;

}

impl<T: Clone> EntityStructImp for T {

    fn clone_entity (&self) -> T {
        return self.clone()
    }
}

pub trait EntityStruct: EntityStructImp {}

#[derive(Clone)]
pub struct MyStruct {}

impl EntityStruct for MyStruct {}



pub struct Entity<'a, E: ?Sized> where E: EntityStruct {

    item: Box<E>,
    id: Uuid,
    base_version: u32,
    toekn: Option<&'a Token>,
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



    #[test]
    fn it_works() {
        let (tx, rx): (Sender<Entity<EntityStruct>>, Receiver<Entity<EntityStruct>>) = channel();
        let v: Vec<Box<Entity<EntityStruct>>> =  Vec::new();



        assert_eq!(2 + 2, 4);
    }
}
