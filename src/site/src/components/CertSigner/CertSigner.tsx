import React, { useCallback } from "react";
import { ISelectionSuccessEvent, PeculiarFortifyCertificates, PeculiarFortifyCertificatesCustomEvent } from 'fortify-webcomponents-react';
import { sign } from "../../libs/certsigner";

interface Props {
    hash: ArrayBuffer;
    onSuccess: (signature: Uint8Array) => void;
}

const CertSigner = (props: Props) => {
    const handleSelectionSuccess = useCallback(async (event: PeculiarFortifyCertificatesCustomEvent<ISelectionSuccessEvent>) => {
        const provider = await event.detail.socketProvider.getCrypto(event.detail.providerId);
        const res = await sign(provider, event.detail.certificateId, event.detail.privateKeyId, props.hash);
        props.onSuccess(new Uint8Array(res));
    }, [props.hash, props.onSuccess]);

    return (
        <PeculiarFortifyCertificates 
            language="en"
            filters={{ onlyWithPrivateKey: true, keyUsage: ['digitalSignature'] }}
            onSelectionSuccess={handleSelectionSuccess}
            onSelectionCancel={() => alert('Very well, but cancel what?')}
        />
    )
};

export default CertSigner;