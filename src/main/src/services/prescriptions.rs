use candid::Principal;
use crate::db::DB;
use crate::db::traits::crud::{CrudSubscribable, Crud};
use crate::models::prescription::{Prescription, PrescriptionId};
use crate::models::prescription_auth::PrescriptionAuthTarget;
use crate::models::user::UserKind;
use crate::utils::vetkd::VetKdUtil;
use crate::utils::x509::X509CertChain;

pub struct PrescriptionsService {}

impl PrescriptionsService {
    pub fn create(
        prescription: &Prescription,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        if *caller == Principal::anonymous() {
            return Err("Anonymous not allowed".to_string());
        }

        // validations
        if let Some(doctor) = db.users.borrow().find_by_id(&caller) {
            match doctor.kind {
                UserKind::Doctor(_) => (),
                _ => return Err("User not a doctor".to_string())
            }
        }
        else {
            return Err("Doctor not found".to_string());
        }
    
        if let Some(patient) = db.users.borrow().find_by_id(&prescription.patient) {
            match patient.kind {
                UserKind::Patient(_) => (),
                _ => return Err("User not a patient".to_string())
            }
        }
        else {
            return Err("Patient not found".to_string());
        }

        // validate the certificate
        let chain = X509CertChain::new(prescription.cert.as_bytes().to_vec());
        match chain.validate() {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        
        db.prescriptions.borrow_mut()
            .insert_and_notify(prescription.id.clone(), prescription.clone())
    }

    pub fn update(
        id: &PrescriptionId,
        req: &Prescription,
        db: &mut DB,
        caller: &Principal
    ) -> Result<(), String> {
        let mut prescriptions = db.prescriptions.borrow_mut();

        let prescription = match prescriptions.find_by_id(id) {
            None => return Err("Not found".to_string()),
            Some(e) => e.clone()
        };
        
        if *caller != prescription.created_by {
            return Err("Forbidden".to_string());
        }

        if prescription.contents.is_some() {
            return Err("A prescription can't be updated after the contents are set".to_string());
        }

        prescriptions.update_and_notify(
            id.to_owned(), 
            Prescription { 
                contents: req.contents.clone(), 
                ..prescription
            }
        )
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
        hash: Vec<u8>,
        encryption_public_key: Vec<u8>,
        vetkd: VetKdUtil
    ) -> Result<String, String> {
        vetkd.get_encrypted_symmetric_key(
            vec![Self::DERIVATION_PATH.to_vec()], 
            hash,
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
                            PrescriptionAuthTarget::User(to) => 
                                if to != *user {
                                    return false;
                                },
                            PrescriptionAuthTarget::Group(to) => {
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

