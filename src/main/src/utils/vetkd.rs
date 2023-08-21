use ic_cdk::export::candid::CandidType;
use ic_cdk::export::serde::Deserialize;
use ic_cdk::export::Principal;
use std::str::FromStr;

#[derive(CandidType, Deserialize, Clone)]
pub enum VetKDCurve {
    #[serde(rename = "bls12_381")]
    Bls12_381,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct VetKDKeyId {
    pub curve: VetKDCurve,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDPublicKeyRequest {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: VetKDKeyId,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDPublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDEncryptedKeyRequest {
    pub public_key_derivation_path: Vec<Vec<u8>>,
    pub derivation_id: Vec<u8>,
    pub key_id: VetKDKeyId,
    pub encryption_public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDEncryptedKeyReply {
    pub encrypted_key: Vec<u8>,
}

#[derive(Clone, CandidType, Deserialize)]
pub struct VetKdUtil {
    pub canister_id: Principal,
    pub key_id: VetKDKeyId,
}

impl Default for VetKdUtil {
    fn default() -> Self {
        Self { 
            canister_id: Principal::anonymous(), 
            key_id: VetKDKeyId {
                curve: VetKDCurve::Bls12_381,
                name: "".to_string(),
            }
        }
    }
}

impl VetKdUtil {
    pub fn new(
        canister_id: String,
        key_name: String
    ) -> Self {
        Self {
            canister_id: Principal::from_str(&canister_id)
                .expect("failed to create canister ID"),
            key_id: VetKDKeyId {
                curve: VetKDCurve::Bls12_381,
                name: key_name,
            }
        }
    }

    pub async fn get_public_key(
        &self,
        derivation_path: Vec<Vec<u8>>
    ) -> Result<String, String> {
        let req = VetKDPublicKeyRequest {
            canister_id: None,
            derivation_path,
            key_id: self.key_id.clone(),
        };
    
        let (res, ): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
            self.canister_id.clone(),
            "vetkd_public_key",
            (req,),
        ).await
        .expect("call to vetkd_public_key failed");
    
        Ok(hex::encode(res.public_key))
    }

    pub async fn get_encrypted_symmetric_key(
        &self,
        public_key_derivation_path: Vec<Vec<u8>>,
        derivation_id: Vec<u8>,
        encryption_public_key: Vec<u8>
    ) -> Result<String, String> {
        let req = VetKDEncryptedKeyRequest {
            derivation_id,
            public_key_derivation_path,
            key_id: self.key_id.clone(),
            encryption_public_key,
        };
    
        let (res,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
            self.canister_id.clone(),
            "vetkd_encrypted_key",
            (req,),
        )
        .await
        .expect("call to vetkd_encrypted_key failed");
    
        Ok(hex::encode(res.encrypted_key))
    }
}
