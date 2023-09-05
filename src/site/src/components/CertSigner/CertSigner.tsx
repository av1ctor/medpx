import React, { useCallback } from "react";
import { ISelectionSuccessEvent, PeculiarFortifyCertificates, PeculiarFortifyCertificatesCustomEvent } from 'fortify-webcomponents-react';
import { sign } from "../../libs/certsigner";
import { useUI } from "../../hooks/ui";

interface Props {
    hash: ArrayBuffer;
    onSuccess: (cert: string, signature: Uint8Array) => void;
}

const CertSigner = (props: Props) => {
    const {showError} = useUI();
    
    const handleSelectionSuccess = useCallback(async (event: PeculiarFortifyCertificatesCustomEvent<ISelectionSuccessEvent>) => {
        const provider = await event.detail.socketProvider.getCrypto(event.detail.providerId);
        const res = await sign(provider, event.detail.certificateId, event.detail.privateKeyId, props.hash);
        props.onSuccess(res.cert, new Uint8Array(res.signature));
    }, [props.hash, props.onSuccess]);

    const handleSelectionCancelled = useCallback(() => {
        showError("Cancelled 😒");
    }, []);

    return (
        <PeculiarFortifyCertificates 
            language="en"
            filters={{ onlyWithPrivateKey: true, keyUsage: ['digitalSignature'] }}
            hideFooter
            onSelectionSuccess={handleSelectionSuccess}
            onSelectionCancel={handleSelectionCancelled}
        />
    )
};

export default CertSigner;