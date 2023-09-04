use std::collections::BTreeMap;

use candid::{CandidType, Principal, decode_args};
use serde::Deserialize;
use crate::{models::{prescription::{PrescriptionId, Prescription}, user::UserId}, db::traits::table::TableData};

#[derive(CandidType, Clone, Deserialize)]
struct PrescriptionV0_1 {
    pub id: PrescriptionId,
    pub doctor: UserId,
    pub patient: UserId,
    pub hash: Vec<u8>,
    pub contents: Option<Vec<u8>>,
    pub created_at: u64,
    pub created_by: Principal,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

pub fn migrate(
    from_version: f32,
    buf: &[u8] 
) -> Result<TableData<PrescriptionId, Prescription>, String> {
    if from_version != 0.1 {
        return Err("Unsupported version".to_string());
    }

    let table = decode_args::<'_, (TableData<PrescriptionId, PrescriptionV0_1>, )>(buf)
        .map_err(|e| format!("{:?}", e))?;

    Ok(
        TableData(
            BTreeMap::from_iter(table.0.0.iter().map(|e| 
                (e.0.clone(), Prescription {
                    id: e.1.id.clone(),
                    doctor: e.1.doctor.clone(),
                    patient: e.1.patient.clone(),
                    hash: e.1.hash.clone(),
                    signature: vec![],
                    contents: e.1.contents.clone(),
                    created_at: e.1.created_at,
                    created_by: e.1.created_by,
                    deleted_at: e.1.deleted_at,
                    deleted_by: e.1.deleted_by,
                })
            ))
        )
    )
}
