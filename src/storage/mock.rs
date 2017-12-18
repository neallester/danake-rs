extern crate uuid;

use std::collections::HashMap;
use self::uuid::Uuid;

pub struct Storage {
    entities: HashMap<String, HashMap<Uuid, String>>
}

impl<> Storage {

    pub fn new() -> Storage {
        Storage {
            entities: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: &str, uuid: &Uuid, json: String) {
        let needs_insert = match self.entities.get_mut (name) {
            Some(mm) => {
                mm.insert(uuid.clone(), json.to_string());
                false
            },
            None => {
                true
            },
        };
        if needs_insert {
            let mut new: HashMap<Uuid, String> = HashMap::new();
            new.insert(uuid.clone(), json);
            self.entities.insert (name.to_string(), new);
        }
    }


    pub fn get (&self, name: &str, uuid: &Uuid) -> Option<String> {
        let map_map = self.entities.get(name);
        match map_map {
            Some (mm) => {
                let json = mm.get (uuid);
                match json {
                    Some (j) => {
                        return Some (j.clone());
                    },
                    None => return None,
                }
            }
            None => return None,
        }
    }

    pub fn table_count (&self) -> usize {
        self.entities.len()
    }

    pub fn count (&self, name: &str) -> usize {
        match self.entities.get (name) {
            Some (mm) => {
                mm.len()
            }
            None => 0,
        }
    }

}


#[cfg(test)]
mod tests {

    use storage::mock::Storage;
    use storage::mock::uuid::Uuid;

    #[test]
    fn storage_entities_tests () {
        let uuid0 = Uuid::parse_str("14e144e3-81ac-4c8c-93c9-e683937a3850").unwrap();
        let uuid1 = Uuid::parse_str("14e144e3-81ac-4c8c-93c9-e683937a3851").unwrap();
        let uuid2 = Uuid::parse_str("14e144e3-81ac-4c8c-93c9-e683937a3852").unwrap();
        let uuid3 = Uuid::parse_str("14e144e3-81ac-4c8c-93c9-e683937a3853").unwrap();
        let json0 = String::from ("{\"value:\":0}");
        let json1 = String::from ("{\"value:\":1}");
        let json2 = String::from ("{\"value:\":2}");
        let json3 = String::from ("{\"value:\":3}");
        let json4 = String::from ("{\"value:\":4}");
        let json5 = String::from ("{\"value:\":5}");
        let name0 = String::from("table0");
        let name1 = String::from("table1");
        let mut storage = Storage::new();
        assert_eq!(None, storage.get (&name0, &uuid0));
        assert_eq!(0, storage.count(&name0));
        assert_eq!(0, storage.table_count());
        storage.set (&name0, &uuid0, json0.clone());
        assert_eq!(json0, storage.get (&name0, &uuid0).unwrap());
        assert_eq! (1, storage.count (&name0));
        assert_eq!(None, storage.get (&name1, &uuid1));
        assert_eq!(0, storage.count(&name1));
        assert_eq!(1, storage.table_count());
        storage.set (&name0, &uuid1, json1.clone());
        storage.set (&name1, &uuid2, json2.clone());
        storage.set (&name1, &uuid3, json3.clone());
        assert_eq!(json0, storage.get (&name0, &uuid0).unwrap());
        assert_eq!(json1, storage.get (&name0, &uuid1).unwrap());
        assert_eq! (2, storage.count (&name0));
        assert_eq!(json2, storage.get (&name1, &uuid2).unwrap());
        assert_eq!(json3, storage.get (&name1, &uuid3).unwrap());
        assert_eq!(2, storage.count(&name1));
        assert_eq!(2, storage.table_count());
        storage.set (&name0, &uuid1, json4.clone());
        storage.set (&name1, &uuid3, json5.clone());
        assert_eq!(json0, storage.get (&name0, &uuid0).unwrap());
        assert_eq!(json4, storage.get (&name0, &uuid1).unwrap());
        assert_eq! (2, storage.count (&name0));
        assert_eq!(json2, storage.get (&name1, &uuid2).unwrap());
        assert_eq!(json5, storage.get (&name1, &uuid3).unwrap());
        assert_eq!(2, storage.count(&name1));
        assert_eq!(2, storage.table_count());
    }
}
