import { UseQueryResult, useMutation, useQuery, useQueryClient } from "react-query";
import { Principal } from "@dfinity/principal";
import { KeyKind, UserRequest, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { userCreate, userDelete, userFindById, userFindByKey, userFindMe, userUpdate } from "../libs/users";

interface UserMethods {
    create: (req: UserRequest) => Promise<UserResponse>;
    update: (id: Principal, req: UserRequest) => Promise<UserResponse>;
    remove: (id: Principal) => Promise<void>;
}

export const useUser = (
): UserMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: UserRequest}) => {
            return userCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['users']);
            }   
        }
    );

    const create = (
        req: UserRequest
    ): Promise<UserResponse> => {
        return createMut.mutateAsync({req});
    };

    const updateMut = useMutation(
        async (options: {id: Principal, req: UserRequest}) => {
            return userUpdate(main, options.id, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['users']);
            }   
        }
    );

    const update = (
        id: Principal,
        req: UserRequest
    ): Promise<UserResponse> => {
        return updateMut.mutateAsync({id, req});
    };

    const deleteMut = useMutation(
        async (options: {id: Principal}) => {
            return userDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['users']);
            }   
        }
    );

    const remove = (
        id: Principal
    ): Promise<void> => {
        return deleteMut.mutateAsync({id});
    };
    
    return {
        create,
        update,
        remove,
    }
};

export const useUserFindMe = (
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', 'me'],
        () => userFindMe(main)
    );
}; 

export const useUserFindById = (
    id: Principal
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', id],
        () => userFindById(main, id)
    );
}; 

export const useUserFindByKey = (
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