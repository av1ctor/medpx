
use std::{vec, collections::HashMap};
use x509_parser::{prelude::{Pem, X509Certificate, Validity, KeyUsage, GeneralName}, x509::X509Name, time::ASN1Time, oid_registry::Oid};

#[derive(Clone)]
pub struct RSAPublicKey {
    pub n: Vec<u8>,
    pub e: Vec<u8>,
}

#[derive(Clone)]
pub struct ECPublicKey {
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub enum PubKeyValue {
    RSA(RSAPublicKey),
    EC(ECPublicKey),
    Unknown,
}

#[derive(Clone)]
pub struct PubKey {
    pub algorithm: String,
    pub value: PubKeyValue,
}

#[derive(Clone)]
pub struct X509Cert{
    pub serial: String,
    pub validity: Validity,
    pub subject: String,
    pub is_ca: bool,
    pub key_usage: Option<KeyUsage>,
    pub pub_key: PubKey,
    pub alt_names: HashMap<String, Vec<u8>>,
}

impl X509Cert {
    pub fn new (
        x509: X509Certificate,
        alt_name_oids: &Vec<Oid<'static>>
    ) -> Self {

        let mut alt_names = HashMap::new();

        if alt_name_oids.len() > 0 {
            match x509.subject_alternative_name() {
                Ok(alt) => {
                    match alt {
                        None => (),
                        Some(ext) => {
                            for name in &ext.value.general_names {
                                if let GeneralName::OtherName(oid, vec) = name {
                                    if alt_name_oids.contains(&oid) {
                                        alt_names.insert(oid.to_id_string(), vec.to_vec());
                                    }
                                }
                            }
                        }
                    }
                },
                Err(_) => (),
            }
        }

        let pub_key = x509.public_key();
        let pub_key_parsed = pub_key.parsed().unwrap();

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
            pub_key: PubKey { 
                algorithm: pub_key.algorithm.algorithm.to_id_string(),
                value: match pub_key_parsed {
                    x509_parser::public_key::PublicKey::RSA(key) => {
                        PubKeyValue::RSA(RSAPublicKey { 
                            n: Self::remove_leading_zero(key.modulus), 
                            e: key.exponent.to_vec()
                        })
                    },
                    x509_parser::public_key::PublicKey::EC(key) => {
                        PubKeyValue::EC(ECPublicKey { 
                            data: key.data().to_vec() 
                        })
                    }
                    //TODO: add support for other public key types
                    _ => PubKeyValue::Unknown
                },
            },
            alt_names,
        }
    }

    fn remove_leading_zero(
        value: &[u8]
    ) -> Vec<u8> {
        if value[0] == 0 {
            value[1..].to_vec()
        }
        else {
            value.to_vec()
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

    pub fn get_top(
        &self
    ) -> Result<X509Cert, String> {
        for cert in &self.chain {
            if cert.check_key_usage().is_ok() {
                return Ok(cert.clone());
            }
        }

        Err("No certificate for digital signature was found".to_string())
    }
}