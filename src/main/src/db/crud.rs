pub trait CRUD<K, V> {
    fn insert(
        &mut self,
        k: &K,
        v: &V
    ) -> Result<(), String>;

    fn find_by_id(
        &self,
        k: &K
    ) -> Option<V>;

    fn get(
        &self,
        k: &K
    ) -> &V;

    fn update(
        &mut self,
        k: &K,
        v: &V
    ) -> Result<(), String>;

    fn delete(
        &mut self,
        k: &K
    ) -> Result<(), String>;
}