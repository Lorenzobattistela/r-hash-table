use std::cmp::PartialEq;
use std::fmt::Debug;

trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut result: usize = 5381;
        for c in self.bytes() {
            result = ((result << 5).wrapping_add(result)).wrapping_add(c.into());
        }
        result
    }
}

#[derive(Default, Clone, Debug)]
struct HashCell<Key, Value> {
    key: Key,
    value: Value,
    taken: bool,
}

#[derive(Debug)]
struct HashTable<Key, Value> {
    cells: Vec<HashCell<Key, Value>>,
    taken_count: usize,
}

impl<Key: Default + Clone + Hashable + Debug + PartialEq, Value: Default + Clone + Debug>
    HashTable<Key, Value>
{
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 11;
        Self {
            cells: vec![HashCell::<_, _>::default(); INITIAL_CAPACITY],
            taken_count: 0,
        }
    }

    fn debug_dump(&self) {
        for cell in self.cells.iter() {
            if cell.taken {
                println!("{:?} -> {:?}", cell.key, cell.value);
            } else {
                println!("X");
            }
        }
    }

    fn extend(&mut self) {
        todo!();
    }

    fn insert(&mut self, key: Key, value: Value) {
        if let Some(old_value) = self.get_mut(&key) {
            *old_value = value.clone();
        }
        // here we call get mut that extends the table anyway

        assert!(self.taken_count < self.cells.len());

        let mut index = key.hash() % self.cells.len();

        while self.cells[index].taken {
            // goes back if it reaches the edge of the array
            index = (index + 1) % self.cells.len();
        }

        self.cells[index].taken = true;
        self.cells[index].key = key;
        self.cells[index].value = value;
        self.taken_count += 1;
    }

    // we do not need to get ownership of the key for get, so use & (a reference for it)
    fn get(&self, key: &Key) -> Option<&Value> {
        let index = key.hash() % self.cells.len();
        if self.cells[index].taken {
            return Some(&self.cells[index].value);
        }
        None
    }

    // now we have a version that returns a mutable version of the value
    fn get_mut(&mut self, key: &Key) -> Option<&mut Value> {
        if self.taken_count >= self.cells.len() {
            self.extend();
        }
        assert!(self.taken_count < self.cells.len());

        let mut index = key.hash() % self.cells.len();
        while self.cells[index].taken && self.cells[index].key != *key {
            index = (index + 1) % self.cells.len();
        }

        // key is equal to the original key
        if self.cells[index].taken {
            assert!(self.cells[index].key == *key);
            return Some(&mut self.cells[index].value);
        }
        None
    }
}

fn main() {
    let mut phone_book = HashTable::<String, String>::new();
    phone_book.insert("a".to_string(), "2983798321".to_string());
    phone_book.debug_dump();
    println!("--------------------------------");
    println!("{:?}", phone_book.get(&"a".to_string()));
    println!("{:?}", phone_book.get(&"b".to_string()));
}
