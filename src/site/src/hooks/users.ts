import { UseQueryResult, useQuery } from "react-query";
import { Principal } from "@dfinity/principal";
import { KeyKind, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { userFindById, userFindByKey, userFindMe } from "../libs/users";
import { useAuth } from "./auth";
import { useCallback, useEffect, useState } from "react";

export const useFindMe = (
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', 'me'],
        () => userFindMe(main)
    );
}; 

export const useFindById = (
    id: Principal
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', id],
        () => userFindById(main, id)
    );
}; 

export const useFindByKey = (
    kind: KeyKind,
    country: [string] | [],
    key: string
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', key],
        () => userFindByKey(main, kind, country, key)
    );
};

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