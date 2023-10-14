use candid::Principal;
use ring::digest::{SHA256, Context};
use ring::signature::{self, RsaPublicKeyComponents};
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription::{Prescription, PrescriptionId, PrescriptionPostRequest, PrescriptionState};
use crate::models::prescription_auth::PrescriptionAuthSubject;
use crate::models::user::UserKind;
use crate::utils::vetkd::VetKdUtil;
use crate::utils::x509::PubKeyValue;
use super::doctors::DoctorsService;

pub struct PrescriptionsService {}

impl PrescriptionsService {
    pub fn pre_create(
        prescription: &Prescription,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }

        // validations
        match db.users.borrow().find_by_id(&caller) {
            None => return Err("Doctor not found".to_string()),
            Some(doctor) => {
                match doctor.kind.clone() {
                    UserKind::Doctor(doctor) => doctor,
                    _ => return Err("User not a doctor".to_string())
                }
            }
        };
    
        if let Some(patient) = db.users.borrow().find_by_id(&prescription.patient) {
            match patient.kind {
                UserKind::Patient(_) => (),
                _ => return Err("User not a patient".to_string())
            }
        }
        else {
            return Err("Patient not found".to_string());
        }

        db.prescriptions.borrow_mut()
            .insert_and_notify(prescription.id.clone(), prescription.clone())
    }

    pub fn post_create(
        id: &PrescriptionId,
        req: &PrescriptionPostRequest,
        db: &mut DB,
        caller: &Principal
    ) -> Result<Prescription, String> {
        let mut prescriptions = db.prescriptions.borrow_mut();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e.clone()
        };
        
        if *caller != prescription.created_by {
            return Err("Forbidden".to_string());
        }

        if prescription.state != PrescriptionState::Created {
            return Err("Invalid prescription state".to_string());
        }

        let doctor = match db.users.borrow().find_by_id(&caller) {
            None => return Err("Doctor not found".to_string()),
            Some(doctor) => {
                match doctor.kind.clone() {
                    UserKind::Doctor(doctor) => doctor,
                    _ => return Err("User not a doctor".to_string())
                }
            }
        };

        // validate the certificate
        if let Err(err) = DoctorsService::validate_cert(
            &req.cert.as_bytes().to_vec(), &doctor) {
            return Err(err);
        }

        // calculate the hash of the encrypted content
        let mut hash = Context::new(&SHA256);
        hash.update(&req.cipher_text.clone());
        let cipher_text_hash = hash.finish();

        if req.cipher_text_hash != cipher_text_hash.as_ref() {
            return Err("The cipher text hash doesn't match".to_string());
        }

        let cert = match DoctorsService::get_top_cert(&req.cert.as_bytes().to_vec()) {
            Ok(cert) => cert,
            Err(err) => return Err(err),
        };

        // validate the signature
        let res = match cert.pub_key.value {
            PubKeyValue::RSA(key) => {
                let pkey = RsaPublicKeyComponents { n: key.n, e: key.e };
                //FIXME: can't assume it's SHA256
                if let Err(err) = pkey.verify(
                    &signature::RSA_PKCS1_2048_8192_SHA256, 
                    cipher_text_hash.as_ref(), 
                    &req.signature
                ) {
                    Some(err.to_string())
                }
                else {
                    None
                }
            },
            PubKeyValue::EC(_) => {
                //FIXME: can't verify EC signatures because ring tries to use assembly: https://github.com/briansmith/ring/issues/918
                /*let pkey = UnparsedPublicKey::new(&signature::ECDSA_P256_SHA256_FIXED, key.data);
                if let Err(err) = pkey.verify(
                    cipher_text_hash.as_ref(), 
                    &req.signature
                ) {
                    return Err(err.to_string());
                }*/
                Some("Unsupported public key type".to_string())
            }
            PubKeyValue::Unknown => {
                Some("Unsupported public key type".to_string())
            }
        };

        // if signature failed, remove the prescription
        if let Some(err) = res {
            _ = prescriptions.delete_and_notify(&prescription.id);
            return Err(err);
        }

        let updated_prescription = Prescription { 
            state: PrescriptionState::Signed,
            cipher_text: Some(req.cipher_text.clone()), 
            cipher_text_hash: Some(cipher_text_hash.as_ref().to_vec()),
            cert: Some(req.cert.clone()),
            ..prescription
        };
        
        // update content and hash only
        if let Err(err) = prescriptions.update_and_notify(
            id.to_owned(), 
            updated_prescription.clone()
        ) {
            return Err(err);
        }

        Ok(updated_prescription)
    }

    pub fn delete(
        id: &PrescriptionId,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut prescriptions = db.prescriptions.borrow_mut();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if *caller != prescription.patient {
            return Err("Forbidden".to_string());
        }
        
        prescriptions.delete_and_notify(id)
    }

    pub fn find_by_id(
        id: &PrescriptionId,
        db: &DB,
        caller: &Principal
    ) -> Result<Prescription, String> {
        let prescriptions = db.prescriptions.borrow();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e
        };

        if prescription.doctor != *caller && prescription.patient != *caller {
            if !Self::has_access(db, &prescription.id, caller) {
                return Err("Forbidden".to_string());
            }
        }

        Ok(prescription.clone())
    }

    const DERIVATION_PATH: &[u8; 13] = b"prescriptions";

    pub async fn get_public_key(
        vetkd: VetKdUtil
    ) -> Result<String, String> {
        vetkd.get_public_key(vec![Self::DERIVATION_PATH.to_vec()])
            .await
    }

    pub async fn get_encrypted_symmetric_key(
        prescription: Prescription,
        encryption_public_key: Vec<u8>,
        vetkd: VetKdUtil
    ) -> Result<String, String> {
        vetkd.get_encrypted_symmetric_key(
            vec![Self::DERIVATION_PATH.to_vec()], 
            prescription.id.as_bytes().to_owned(),
            encryption_public_key
        ).await
    }

    pub fn has_access(
        db: &DB, 
        prescription_id: &PrescriptionId, 
        user: &Principal
    ) -> bool {
        let now = ic_cdk::api::time();
        match db.prescription_auths_rel.borrow().find_by_id(prescription_id) {
            None => return false,
            Some(ids) => {
                if !ids.iter().map(|id| 
                    db.prescription_auths.borrow().get(id).clone()
                    ).any(|e| {
                        if let Some(expiration) = e.expires_at {
                            if now > expiration {
                                return false;
                            }
                        }
                        
                        match e.to {
                            PrescriptionAuthSubject::User(to) => 
                                if to != *user {
                                    return false;
                                },
                            PrescriptionAuthSubject::Group(to) => {
                                match db.groups.borrow().find_by_id(&to) {
                                    None => return false,
                                    Some(group) => return group.members.contains(user)
                                }
                            },
                        }

                        true
                    }) {
                        return false;
                }
            }
        }
        true
    }
}

