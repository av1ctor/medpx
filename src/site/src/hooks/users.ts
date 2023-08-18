import { UseQueryResult, useQuery } from "react-query";
import { Principal } from "@dfinity/principal";
import { UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { userFindById, userFindByKey, userFindMe } from "../libs/users";

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
    key: string
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', key],
        () => userFindByKey(main, key)
    );
}; 