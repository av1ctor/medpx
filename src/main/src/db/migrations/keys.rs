use std::collections::BTreeMap;

use candid::{CandidType, Principal, decode_args};
use serde::Deserialize;
use crate::{models::key::{KeyId, Key, KeyKind}, sdb::table::TableData};

#[derive(CandidType, Clone, Deserialize)]
struct KeyV0_1 {
    pub id: KeyId,
    pub country: String,
    pub kind: KeyKind,
    pub value: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

pub fn migrate(
    from_version: f32,
    buf: &[u8] 
) -> Result<TableData<KeyId, Key>, String> {
    if from_version != 0.1 {
        return Err("Unsupported version".to_string());
    }

    let table = decode_args::<'_, (TableData<KeyId, KeyV0_1>, )>(buf)
        .map_err(|e| format!("{:?}", e))?;

    Ok(
        TableData(
            BTreeMap::from_iter(table.0.0.iter().map(|e| 
                (e.0.clone(), Key {
                    id: e.1.id.clone(),
                    kind: e.1.kind.clone(),
                    country: Some(e.1.country.clone()),
                    value: e.1.value.clone(),
                    created_at: e.1.created_at,
                    created_by: e.1.created_by,
                    updated_at: e.1.updated_at,
                    updated_by: e.1.updated_by,
                    deleted_at: e.1.deleted_at,
                    deleted_by: e.1.deleted_by,
                })
            ))
        )
    )
}
