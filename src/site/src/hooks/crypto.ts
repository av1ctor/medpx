import { Principal } from "@dfinity/principal";
import { useAuth } from "./auth";
import { useCallback, useEffect, useState } from "react";

export interface DecryptResult {
    Ok: string|undefined;
    Err: string|undefined;
}

export const useDecrypt = (
    message: Uint8Array,
    principal: Principal,
    isEncrypted: number
): DecryptResult => {
    const {aes_gcm} = useAuth();
    const [text, setText] = useState<string|undefined>();
    const [err, setErr] = useState<string|undefined>();
    
    const decrypt = useCallback(async (        
    ): Promise<void> => {
        if(!isEncrypted) {
            setText(new TextDecoder().decode(message));
            return;
        }

        if(!aes_gcm) {
            return;
        }

        const rawKey = await aes_gcm.genRawKey('prescriptions', principal);
        if('Err' in rawKey || !rawKey.Ok) {
            setErr('Raw key generation failed');
            return;
        }
        
        try {
            setText(await aes_gcm.decrypt(message, rawKey.Ok));
        }
        catch(e: any) {
            setErr(e.message || "Call to AES GCM decrypt failed");
        }

    }, [aes_gcm, message, principal, isEncrypted]);
    
    useEffect(() => {
        decrypt();
    }, [decrypt])

    return {
        Ok: text,
        Err: err,
    };
}