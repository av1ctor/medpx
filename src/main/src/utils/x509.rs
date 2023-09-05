
use std::{vec, collections::HashMap};
use x509_parser::{prelude::{Pem, X509Certificate, Validity, KeyUsage, GeneralName}, x509::X509Name, time::ASN1Time, oid_registry::Oid};

pub struct X509Cert{
    pub serial: String,
    pub validity: Validity,
    pub subject: String,
    pub is_ca: bool,
    pub key_usage: Option<KeyUsage>,
    pub extensions: HashMap<String, Vec<u8>>,
}

impl X509Cert {
    pub fn new (
        x509: X509Certificate,
        alt_name_oids: &Vec<Oid<'static>>
    ) -> Self {

        let mut extensions = HashMap::new();

        match x509.subject_alternative_name() {
            Ok(alt) => {
                match alt {
                    None => (),
                    Some(ext) => {
                        for name in &ext.value.general_names {
                            if let GeneralName::OtherName(oid, vec) = name {
                                if alt_name_oids.contains(&oid) {
                                    extensions.insert(oid.to_id_string(), vec.to_vec());
                                }
                            }
                        }
                    }
                }
            },
            Err(_) => (),
        }
        
        Self { 
            serial: x509.serial.clone().to_str_radix(16),
            validity: x509.validity.clone(),
            subject: Self::get_cn(&x509.subject),
            is_ca: x509.is_ca(),
            key_usage: match x509.key_usage() {
                Ok(res) => match res {
                    None => None,
                    Some(be) => Some(be.value.clone())    
                },
                Err(_) => None
            },
            extensions,
        }
    }
    
    pub fn check_validity(
        &self
    ) -> Result<(), String> {
        let now = ASN1Time::from_timestamp((ic_cdk::api::time() / 1000_000_000) as i64).unwrap();
        if !self.validity.is_valid_at(now) {
            return Err("Certificate expired".to_string());
        }

        Ok(())
    }

    pub fn check_key_usage(
        &self
    ) -> Result<(), String> {
        match self.key_usage {
            None => return Err("Certificate can't be used to sign".to_string()),
            Some(ku) => if !ku.digital_signature() {
                return Err("Certificate can't be used to sign".to_string());
            }
        }

        Ok(())
    }

    fn get_cn(
        name: &X509Name
    ) -> String {
        let cn = name.iter_common_name()
            .next()
            .and_then(|cn| cn.as_str().ok()).unwrap().to_string();

        if let Some(p) = cn.find(":") {
            if p == 0 {
                "".to_string()
            } 
            else {
                cn.get(0..p).unwrap().to_string()
            }
        }
        else {
            cn
        }
    }
}

pub struct X509CertChain {
    pub chain: Vec<X509Cert>,
}

impl X509CertChain {
    pub fn new(
        buffer: &Vec<u8>,
        alt_name_oids: &Vec<Oid<'static>>
    ) -> Self {
        let mut chain = vec![];

        for pem in Pem::iter_from_buffer(&buffer) {
            let pem = pem.expect("Reading next PEM block failed");

            let x509 = pem.parse_x509().expect("X.509: decoding DER failed");

            let cert = X509Cert::new(x509, alt_name_oids);

            chain.push(cert);
        }
        
        Self { 
            chain
        }
    }

    pub fn validate(
        &self
    ) -> Result<&X509Cert, String> {
        let mut top = None;
        
        for cert in &self.chain {
            if let Err(err) = cert.check_validity() {
                return Err(err);
            }
            
            if let Err(err) = cert.check_key_usage() {
                if self.chain.len() == 1 {
                    return Err(err);
                }
            } 
            else {
                top = Some(cert);
            }

            //TODO: validate the chain (check if CA is trusted and if it was not revoked)

            //TODO: check this cert CRL ("offline")
        }

        if top.is_none() {
            return Err("No certificate for digital signature was found".to_string());
        }

        Ok(top.unwrap())
    }
}