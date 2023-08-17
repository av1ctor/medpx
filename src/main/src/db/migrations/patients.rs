use std::collections::BTreeMap;

use candid::{CandidType, Principal, decode_args};
use serde::Deserialize;
use crate::{models::patient::{PatientId, Patient}, db::traits::table::TableData};

#[derive(CandidType, Clone, Deserialize)]
struct PatientV0_1 {
    pub id: PatientId,
    pub name: String,
    pub birth_date: u64,
    pub credits: u128,
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
) -> Result<TableData<PatientId, Patient>, String> {
    if from_version != 0.1 {
        return Err("Unsupported version".to_string());
    }

    let table = decode_args::<'_, (TableData<PatientId, PatientV0_1>, )>(buf)
                .map_err(|e| format!("{:?}", e))?;

    Ok(
        TableData(
            BTreeMap::from_iter(table.0.0.iter().map(|e| 
                (*e.0, Patient {
                    id: e.1.id,
                    name: e.1.name.clone(),
                    email: "".to_string(),
                    birth_date: e.1.birth_date,
                    credits: e.1.credits,
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