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

pub type TableEventCallback = fn (key: TableEventKey) -> ();

#[derive(CandidType, Deserialize)]
pub struct TableData<K, V> (pub BTreeMap<K, V>)
    where K: Ord + CandidType, V: CandidType, Self: Sized;

pub struct Table<K, V> 
    where K: Ord + CandidType, V: CandidType {
    pub data: TableData<K, V>,
    pub subs: BTreeMap<TableEventKind, Vec<TableEventCallback>>,
}

pub trait TableAllocator<K: Ord + CandidType, V: CandidType> {
    fn new(
    ) -> Table<K, V> {
        Table { 
            data: TableData(BTreeMap::new()), 
            subs: BTreeMap::new(), 
        }
    }
}

pub trait TableSerializer<K: Ord + CandidType, V: CandidType> {
    fn serialize(
        table: &Table<K, V>,
        writer: &mut StableWriter
    ) -> Result<(), String> {
        let mut ser = IDLBuilder::new();
        (&table.data, ).encode(&mut ser).map_err(|e| format!("{:?}", e))?;
        let arr = ser.serialize_to_vec().unwrap();
        // store size
        writer.write(&u64::to_le_bytes(arr.len() as u64));
        // store table
        writer.write(&arr);
        Ok(())
    }
}

pub trait TableDeserializer<K: Ord + CandidType + for<'a> Deserialize<'a>, V: CandidType + for<'a> Deserialize<'a>> {
    fn deserialize(
        table: &Table<K, V>,
        reader: &mut StableReader
    ) -> Result<TableData<K, V>, String> {
        // load size
        let mut size_buf = [0u8; 8];
        reader.read(&mut size_buf).map_err(|e| format!("{:?}", e))?;
        let size = u64::from_le_bytes(size_buf);
        // load table
        let mut bytes = vec![0u8; size as usize];
        reader.read(&mut bytes).map_err(|e| format!("{:?}", e))?;
        // decode table
        let res = candid::decode_args::<'_, (TableData<K, V>, )>(&bytes)
            .map_err(|e| format!("{:?}", e))?;
        Ok(res.0)
    }
}

pub trait TableSubscribed<K: Ord + CandidType, V: CandidType> {
    fn alert (
        subs: &BTreeMap<TableEventKind, Vec<TableEventCallback>>,
        kind: TableEventKind,
        k: TableEventKey
    ) -> () {
        subs.get(&kind).unwrap().iter()
            .for_each(|f| f(k.clone()));
    }
}