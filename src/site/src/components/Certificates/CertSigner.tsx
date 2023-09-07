import React, { useCallback } from "react";
import { ISelectionSuccessEvent, PeculiarFortifyCertificates, PeculiarFortifyCertificatesCustomEvent } from '@peculiar/fortify-webcomponents-react';
import { sign } from "../../libs/certificates";
import { useUI } from "../../hooks/ui";

interface Props {
    hash: ArrayBuffer;
    onSuccess: (cert: string, signature: Uint8Array) => Promise<void>;
}

const CertSigner = (props: Props) => {
    const {showError, toggleLoading} = useUI();
    
    const handleSelectionSuccess = useCallback(async (event: PeculiarFortifyCertificatesCustomEvent<ISelectionSuccessEvent>) => {
        try {
            toggleLoading(true);
            const provider = await event.detail.socketProvider.getCrypto(event.detail.providerId);
            const res = await sign(provider, event.detail.certificateId, event.detail.privateKeyId, props.hash);
            await props.onSuccess(res.cert, new Uint8Array(res.signature));
        }
        catch(e) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
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