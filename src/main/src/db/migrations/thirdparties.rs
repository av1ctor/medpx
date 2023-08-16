use std::collections::BTreeMap;

use candid::{CandidType, Principal, decode_args};
use serde::Deserialize;
use crate::{models::thirdparty::{ThirdPartyId, ThirdParty, ThirdPartyKind}, db::traits::table::TableData};

#[derive(CandidType, Clone, Deserialize)]
struct ThirdPartyV0_1 {
    pub id: ThirdPartyId,
    pub kind: ThirdPartyKind,
    pub name: String,
    pub created_at: u64,
    pub created_by: Principal,
    pub updated_at: Option<u64>,
    pub updated_by: Option<Principal>,
    pub deleted_at: Option<u64>,
    pub deleted_by: Option<Principal>,
}

#[derive(CandidType, Clone, Deserialize)]
enum ThirdPartyKindV0_2 {
    Hospital,
    DrogStore,
    Other,
}

#[derive(CandidType, Clone, Deserialize)]
struct ThirdPartyV0_2 {
    pub id: ThirdPartyId,
    pub kind: ThirdPartyKindV0_2,
    pub name: String,
    pub email: String,
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
) -> Result<TableData<ThirdPartyId, ThirdParty>, String> {
    if from_version == 0.1 {
        let table = decode_args::<'_, (TableData<ThirdPartyId, ThirdPartyV0_1>, )>(buf)
        .map_err(|e| format!("{:?}", e))?;

        Ok(
            TableData(
                BTreeMap::from_iter(table.0.0.iter().map(|e| 
                    (*e.0, ThirdParty {
                        id: e.1.id,
                        kind: e.1.kind.clone(),
                        name: e.1.name.clone(),
                        email: "".to_string(),
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
    else if from_version == 0.2 {
        let table = decode_args::<'_, (TableData<ThirdPartyId, ThirdPartyV0_2>, )>(buf)
        .map_err(|e| format!("{:?}", e))?;

        Ok(
            TableData(
                BTreeMap::from_iter(table.0.0.iter().map(|e| 
                    (*e.0, ThirdParty {
                        id: e.1.id,
                        kind: match e.1.kind {
                            ThirdPartyKindV0_2::Hospital => ThirdPartyKind::Hospital,
                            ThirdPartyKindV0_2::DrogStore => ThirdPartyKind::DrugStore,
                            ThirdPartyKindV0_2::Other => ThirdPartyKind::Other,
                        },
                        name: e.1.name.clone(),
                        email: e.1.email.clone(),
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
    else {
        return Err("Unsupported version".to_string());
    }

}
