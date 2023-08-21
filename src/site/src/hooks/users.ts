import { UseQueryResult, useQuery } from "react-query";
import { Principal } from "@dfinity/principal";
import { KeyKind, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { userFindById, userFindByKey, userFindMe } from "../libs/users";
import { useAuth } from "./auth";
import { useState } from "react";

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
    message: Uint8Array
): DecryptResult => {
    const {aes_gcm} = useAuth();
    const [text, setText] = useState<string|undefined>();
    const [err, setErr] = useState<string|undefined>();

    aes_gcm?.decrypt(message).then((value: string) => {
        setText(value);
    }, 
    (reason: any) => {
        setErr(reason);
    });

    return {
        Ok: text,
        Err: err,
    };
}