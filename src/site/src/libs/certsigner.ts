import {Certificate, ContentInfo, CryptoEngine, EncapsulatedContentInfo, IssuerAndSerialNumber, SignedData, SignerInfo} from 'pkijs';
import {OctetString, fromBER} from 'asn1js';
import { SocketCrypto } from 'fortify-webcomponents-react';

export const sign = async (
    provider: SocketCrypto,
    certificateId: string,
    privateKeyId: string,
    hash: BufferSource
): Promise<ArrayBuffer> => {
    const crypto = new CryptoEngine({ crypto: provider });

    const cert = await provider.certStorage.getItem(certificateId);
    const privateKey = await provider.keyStorage.getItem(privateKeyId);
    const certRawData = await provider.certStorage.exportCert('raw', cert);

    const pkiCert = new Certificate({
        schema: fromBER(certRawData).result,
    });

    const signedData = new SignedData({
        version: 1,
        encapContentInfo: new EncapsulatedContentInfo({
            eContentType: "1.2.840.113549.1.7.1", // "data" content type
        }),
        signerInfos: [
            new SignerInfo({
                version: 1,
                sid: new IssuerAndSerialNumber({
                    issuer: pkiCert.issuer,
                    serialNumber: pkiCert.serialNumber,
                }),
            }),
        ],
        certificates: [pkiCert],
    });

    const contentInfo = new EncapsulatedContentInfo({
        eContent: new OctetString({
            valueHex: hash,
        }),
    });

    signedData.encapContentInfo.eContent = contentInfo.eContent;

    await signedData.sign(privateKey, 0, "sha-256", undefined, crypto);

    const cms = new ContentInfo({
        contentType: "1.2.840.113549.1.7.2",
        content: signedData.toSchema(true),
    });
    
    return cms.toSchema().toBER(false);
};