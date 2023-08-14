use candid::CandidType;
use super::table::{TableSubscribable, TableEventKind, Table};

pub trait Crud<K, V> 
    where 
        K: Ord + CandidType, 
        V: CandidType, 
        Self: Table<K, V> {
    
    fn insert(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if self.get_data().0.contains_key(&k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn find_by_id<'a>(
        &'a self,
        k: &'a K
    ) -> Option<&'a V> {
       self.get_data().0.get(k)
    }

    fn get<'a>(
        &'a self,
        k: &'a K
    ) -> &'a V {
        self.get_data().0.get(k).unwrap()
    }

    fn update(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if !self.get_data().0.contains_key(&k) {
            Err("Not found".to_string())
        }
        else {
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn delete(
        &mut self,
        k: &K
    ) -> Result<(), String> {
        _ = self.get_data_mut().0.remove(k);
        Ok(())
    }
}

pub trait CrudSubscribable<K, V> 
    where 
        K: Ord + CandidType, 
        V: CandidType, 
        Self: Table<K, V> + TableSubscribable<K, V> {
    fn insert(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if self.get_data().0.contains_key(&k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.notify(TableEventKind::Create, Self::get_keys(&k, &v));
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn find_by_id<'a>(
        &'a self,
        k: &'a K
    ) -> Option<&'a V> {
       self.get_data().0.get(k)
    }

    fn get<'a>(
        &'a self,
        k: &'a K
    ) -> &'a V {
        self.get_data().0.get(k).unwrap()
    }

    fn update(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if !self.get_data().0.contains_key(&k) {
            Err("Not found".to_string())
        }
        else {
            self.notify(TableEventKind::Update, Self::get_keys(&k, &v));
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn delete(
        &mut self,
        k: K
    ) -> Result<(), String> {
        let v = self.get_data_mut().0.remove(&k);
        if let Some(v) = v {
            self.notify(TableEventKind::Delete, Self::get_keys(&k, &v));
        }
        Ok(())
    }
}

