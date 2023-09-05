import React, { useCallback } from "react";
import { ISelectionSuccessEvent, PeculiarFortifyCertificates, PeculiarFortifyCertificatesCustomEvent } from 'fortify-webcomponents-react';
import { retrieve } from "../../libs/certificates";
import { useUI } from "../../hooks/ui";

interface Props {
    onSuccess: (cert: string, serial: string) => void;
}

const CertSeletor = (props: Props) => {
    const {showError} = useUI();
    
    const handleSelectionSuccess = useCallback(async (event: PeculiarFortifyCertificatesCustomEvent<ISelectionSuccessEvent>) => {
        const provider = await event.detail.socketProvider.getCrypto(event.detail.providerId);
        const res = await retrieve(provider, event.detail.certificateId);
        props.onSuccess(res.cert, res.serial);
    }, [props.onSuccess]);

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

export default CertSeletor;