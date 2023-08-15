use candid::CandidType;
use serde::Deserialize;
use super::table::{TableSubscribable, TableEventKind, Table, TableEvent};


#[derive(CandidType, Deserialize)]
pub struct Pagination {
    pub offset: u32,
    pub limit: u32,
}

pub trait Crud<TN, K, V> 
    where 
        K: Ord + CandidType, 
        V: CandidType, 
        Self: Table<TN, K, V> {
    
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

pub trait CrudSubscribable<TN, K, V> 
    where 
        K: Ord + CandidType, 
        V: CandidType, 
        Self: Table<TN, K, V> + TableSubscribable<TN, K, V> {
    fn insert(
        &mut self,
        k: K,
        v: V
    ) -> Result<(), String> {
        if self.get_data().0.contains_key(&k) {
            Err("Duplicated key".to_string())
        }
        else {
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Create, 
                pkey: Self::get_pkey(&k),
                keys: Self::get_keys(&v)
            });
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
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Update, 
                pkey: Self::get_pkey(&k),
                keys: Self::get_keys(&v)
            });
            self.get_data_mut().0.insert(k, v);
            Ok(())
        }
    }

    fn delete(
        &mut self,
        k: &K
    ) -> Result<(), String> {
        let v = self.get_data_mut().0.remove(k);
        if let Some(v) = v {
            self.notify(&TableEvent {
                table_name: &self.get_schema().name,
                kind: TableEventKind::Delete, 
                pkey: Self::get_pkey(k),
                keys: Self::get_keys(&v)
            });
        }
        Ok(())
    }
}

