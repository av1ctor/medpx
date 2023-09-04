import React from "react";
import { PeculiarFortifyCertificates } from 'fortify-webcomponents-react';

const SmartCardSigner = () => {
    const handleSelectionSuccess = () => {
        // Happy
      };

    return (
        <PeculiarFortifyCertificates 
            language="en"
            filters={{ onlySmartcards: true, onlyWithPrivateKey: true, keyUsage: ['digitalSignature'] }}
            onSelectionSuccess={handleSelectionSuccess}
            onSelectionCancel={() => alert('Very well, but cancel what?')}
        />
    )
};

export default SmartCardSigner;