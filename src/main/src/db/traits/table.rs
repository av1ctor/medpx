use std::collections::BTreeMap;
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
    where K: Ord + CandidType, V: CandidType, Self: Sized;

pub struct Table<'a, K, V> 
    where K: Ord + CandidType, V: CandidType {
    pub data: TableData<K, V>,
    pub subs: Vec<&'a mut dyn TableSubscriber>,
}
pub trait TableAllocatable<'a, K: Ord + CandidType, V: CandidType> {
    fn new(
    ) -> Table<'a, K, V> {
        Table { 
            data: TableData(BTreeMap::new()), 
            subs: Vec::new(), 
        }
    }
}
pub trait TableSerializable<K: Ord + CandidType, V: CandidType> {
    fn serialize(
        table: &Table<K, V>,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        let mut ser = IDLBuilder::new();
        (&table.data, ).encode(&mut ser).map_err(|e| format!("{:?}", e))?;
        let arr = ser.serialize_to_vec().unwrap();
        // store size
        writer.write(&u64::to_le_bytes(arr.len() as u64)).map_err(|e| format!("{:?}", e))?;
        // store table
        writer.write(&arr).map_err(|e| format!("{:?}", e))?;
        Ok(())
    }
}

pub trait TableDeserializable<K: Ord + CandidType + for<'a> Deserialize<'a>, V: CandidType + for<'a> Deserialize<'a>> {
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
        key: TableEventKey
    );
}
pub trait TableSubscribable<'a, K: Ord + CandidType, V: CandidType> {
    fn subscribe(
        &'a mut self,
        tb: &'static mut dyn TableSubscriber
    );

    fn notify (
        subs: &mut Vec<&'a mut dyn TableSubscriber>,
        kind: TableEventKind,
        key: TableEventKey
    ) {
        subs.iter_mut().for_each(|c| c.on(kind.clone(), key.clone()));
    }
}

