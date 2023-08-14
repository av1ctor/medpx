use std::{collections::BTreeMap, cell::RefCell, rc::Rc};
use candid::{CandidType, ser::IDLBuilder, utils::ArgumentEncoder, Principal};
use ic_cdk::api::stable::{StableWriter, StableReader};
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableEventKind {
    Create,
    Update,
    Delete
}

#[derive(Clone)]
pub enum TableEventKey {
    Text(String),
    Principal(Principal),
}

#[derive(CandidType, Deserialize)]
pub struct TableData<K, V> (pub BTreeMap<K, V>)
    where K: Ord + CandidType, V: CandidType;

pub struct TableSubs (pub Vec<Rc<RefCell<dyn TableSubscriber>>>);

pub trait TableAllocatable<T> {
    fn new(
    ) -> T;
}

pub trait TableSerializable<K, V>
    where K: Ord + CandidType, V: CandidType {
    fn serialize(
        data: &TableData<K, V>,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        let mut ser = IDLBuilder::new();
        (data, ).encode(&mut ser).map_err(|e| format!("{:?}", e))?;
        let arr = ser.serialize_to_vec().unwrap();
        // store size
        writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| format!("{:?}", e))?;
        // store table
        writer.write(&arr).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}

pub trait TableDeserializable<K, V>
    where K: Ord + CandidType + for<'a> Deserialize<'a>, 
          V: CandidType + for<'a> Deserialize<'a> {
    fn deserialize(
        reader: &mut StableReader
    ) -> Result<TableData<K, V>, String> {
        // load size
        let mut size_buf = [0u8; 8];
        reader.read(&mut size_buf).map_err(|e| format!("{:?}", e))?;
        let size = u64::from_le_bytes(size_buf);
        // load table
        let mut table_buf = vec![0u8; size as usize];
        reader.read(&mut table_buf).map_err(|e| format!("{:?}", e))?;
        // decode table
        let res = candid::decode_args::<'_, (TableData<K, V>, )>(&table_buf)
            .map_err(|e| format!("{:?}", e))?;
        Ok(res.0)
    }
}
pub trait TableSubscriber {
    fn on(
        &mut self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    );
}
pub trait TableSubscribable  {
    fn get_subs(
        &self
    ) -> &TableSubs;

    fn get_subs_mut(
        &mut self
    ) -> &mut TableSubs;

    fn subscribe(
        &mut self,
        tb: Rc<RefCell<dyn TableSubscriber>>
    ) {
        self.get_subs_mut().0.push(tb);
    }

    fn notify (
        &self,
        kind: TableEventKind,
        keys: Vec<TableEventKey>
    ) {
        self.get_subs().0.iter()
            .for_each(|c| c.borrow_mut().on(kind.clone(), keys.clone()));
    }
}
