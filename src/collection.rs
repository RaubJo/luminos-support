
#[derive(Debug, Clone, Default)]
pub struct Collection<T> {
    items: Vec<T>,
}

#[allow(clippy::iter_overeager_cloned, clippy::redundant_closure)]
impl<T> Collection<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Create a collection from a vec.
    pub fn from_vec(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Push an item onto the collection
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// Get all items in the collection.
    pub fn all(&self) -> &Vec<T> {
        &self.items
    }

    /// Get the items off the collection.
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
    
    /// Get the length of the collection
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Get the length of the collection
    /// Idiomatic rust is to use len()
    pub fn length(&self) -> usize {
        self.len()
    }

    /// Is the collection empty?
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Get an item by index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Filter the collection by the closure.
    /// False(y) items will be removec
    pub fn filter<F>(&self, mut f: F) -> Self
    where
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        let filtered = self.items.iter().cloned().filter(|item| f(item)).collect();
        Self::from_vec(filtered)
    }

    /// Map over the collection.
    pub fn map<U, F>(&self, mut f: F) -> Collection<U>
    where
        F: FnMut(&T) -> U,
    {
        let mapped = self.items.iter().map(|item| f(item)).collect();
        Collection::from_vec(mapped)
    }
}
