
use x509_parser::{prelude::{Pem, X509Certificate}, x509::X509Name, time::ASN1Time};

pub struct X509Cert<'a>{
    pub cert: X509Certificate<'a>,
}

impl X509Cert<'_> {
    pub fn validate(
        &self
    ) -> Result<(), String> {
        let now = ASN1Time::from_timestamp((ic_cdk::api::time() / 1000_000_000) as i64).unwrap();
        if !self.cert.validity.is_valid_at(now) {
            return Err("Certificate expired".to_string());
        }

        ic_cdk::print(format!("subject: {}", Self::get_cn(&self.cert.subject)));

        Ok(())
    }

    pub fn get_cn(
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
    buffer: Vec<u8>,
}

impl X509CertChain {
    pub fn new(
        buffer: Vec<u8>
    ) -> Self {
        Self { 
            buffer
        }
    }

    pub fn validate(
        &self
    ) -> Result<(), String> {
        for pem in Pem::iter_from_buffer(&self.buffer) {
            let pem = pem.expect("Reading next PEM block failed");

            let x509 = pem.parse_x509().expect("X.509: decoding DER failed");

            let cert = X509Cert {
                cert: x509
            };

            if let Err(err) = cert.validate() {
                return Err(err);
            }
        }

        Ok(())
    }
}