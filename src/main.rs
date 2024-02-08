trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut result: usize = 5381;
        for c in self.bytes() {
            result = ((result << 5).wrapping_add(result)).wrapping_add(c as usize);
        }
        result
    }
}

#[derive(Debug)]
struct HashTable<Key, Value> {
    key_vals: Vec<(Key, Value)>,
}

impl<Key: Default + Clone + Hashable, Value: Default + Clone> HashTable<Key, Value> {
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 64;
        Self {
            key_vals: vec![(Key::default(), Value::default()); INITIAL_CAPACITY],
        }
    }

    fn insert(key: Key, value: Value) {
        todo!()
    }

    // we do not need to get ownership of the key for get, so use & (a reference for it)
    fn get(key: &Key) -> &Value {
        todo!()
    }

    // now we have a version that returns a mutable version of the value
    fn get_mut(key: &Key) -> &mut Value {
        todo!()
    }
}

fn main() {
    println!("{}", "Hello, World".to_string().hash())
}
