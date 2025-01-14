use super::hash_map::HashMap;

pub struct Table<K> {
    table: HashMap<K, isize>,
    current_index: isize,
}

#[allow(dead_code)]
impl<K> Table<K>
where
    K: std::fmt::Debug + Clone + Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            current_index: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.table.size()
    }

    pub fn insert(&mut self, key: K) -> isize {
        self.table.put(key, self.current_index);
        self.current_index += 1;

        self.current_index - 1
    }

    pub fn put(&mut self, key: K, value: isize) {
        self.table.put(key, value);
    }

    pub fn get(&self, key: &K) -> Option<&isize> {
        self.table.get(key)
    }

    pub fn remove(&mut self, key: &K) {
        self.table.remove(key)
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }

    pub fn display(&self) {
        self.table.display();
    }
}
