use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash_map<K: Hash, V: Hash>(map: &HashMap<K, V>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for (key, value) in map {
        key.hash(&mut hasher);
        value.hash(&mut hasher);
    }
    hasher.finish()
}

pub fn collation(
    map1: &HashMap<String, HashMap<i8, String>>,
    map2: &HashMap<String, HashMap<i8, String>>
) -> bool {
    if map1.len() != map2.len() {
        return false;
    }

    for (key, value1) in map1 {
        match map2.get(key) {
            Some(value2) if hash_map(value1) == hash_map(value2) => continue,
            _ => return false,
        }
    }

    true
}