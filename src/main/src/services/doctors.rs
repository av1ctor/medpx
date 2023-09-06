use x509_parser::der_parser::oid;
use x509_parser::oid_registry::Oid;
use crate::models::doctor::Doctor;
use crate::utils::x509::{X509CertChain, X509Cert};

pub struct DoctorsService {}

const ICP_BRASIL_PERSON_DATA: Oid<'static> = oid!(2.16.76.1.3.1);

impl DoctorsService {
    pub fn validate_cert(
        x509: &Vec<u8>,
        doctor: &Doctor
    ) -> Result<X509Cert, String> {
        let chain = X509CertChain::new(
            x509, 
            &vec![
                ICP_BRASIL_PERSON_DATA
            ]
        );
        
        let cert = match chain.validate() {
            Ok(cert) => cert,
            Err(err) => return Err(err),
        };

        //TODO: add support for other countries
        
        //FIXME: we should verify the doctor's license, not the subject's national ID
        match cert.alt_names.get(&ICP_BRASIL_PERSON_DATA.to_id_string()) {
            Some(person_data) => {
                let license_num = String::from_utf8(person_data[4+8..4+8+11].to_vec()).unwrap_or_default();
                if doctor.license_num != license_num {
                    return Err("Certificate license number is different from doctor's one".to_string());
                }
            },
            None => return Err("Certificate doesn't contain a license number".to_string()),
        };

        return Ok(cert.clone())
    }
}