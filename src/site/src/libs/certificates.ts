import {CryptoEngine, Certificate} from 'pkijs';
import { SocketCrypto } from '@peculiar/fortify-webcomponents-react';

export const sign = async (
    provider: SocketCrypto,
    certificateId: string,
    privateKeyId: string,
    hash: BufferSource
): Promise<{cert: string, signature: ArrayBuffer}> => {
    const crypto = new CryptoEngine({ crypto: provider });

    const cert = await provider.certStorage.getItem(certificateId);
    const privateKey = await provider.keyStorage.getItem(privateKeyId);
    const certPem = await provider.certStorage.exportCert('pem', cert);
    const signature = await crypto.sign(cert.publicKey.algorithm, privateKey, hash);

    return {
        cert: certPem, 
        signature
    };
};

export const retrieve = async (
    provider: SocketCrypto,
    certificateId: string,
): Promise<{cert: string, serial: string}> => {
    
    const cert = await provider.certStorage.getItem(certificateId);
    const certPem = await provider.certStorage.exportCert('pem', cert);
    const certRaw = await provider.certStorage.exportCert('raw', cert);

    const x509 = Certificate.fromBER(certRaw);

    return {
        cert: certPem, 
        serial: x509.serialNumber.toString('hex'),
    };
};