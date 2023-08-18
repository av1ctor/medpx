import { UseInfiniteQueryResult, useInfiniteQuery, useMutation, useQueryClient } from "react-query";
import { KeyRequest, KeyResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { keyCreate, keyDelete, keyFindByUser } from "../libs/keys";
import { userGetPrincipal } from "../libs/users";

interface KeyMethods {
    create: (req: KeyRequest) => Promise<KeyResponse>;
    remove: (id: string) => Promise<void>;
}

export const useKey = (
): KeyMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: KeyRequest}) => {
            return keyCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['keys']);
            }   
        }
    );

    const create = (
        req: KeyRequest
    ): Promise<KeyResponse> => {
        return createMut.mutateAsync({req});
    };

    const deleteMut = useMutation(
        async (options: {id: string}) => {
            return keyDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['keys']);
            }   
        }
    );

    const remove = (
        id: string
    ): Promise<void> => {
        return deleteMut.mutateAsync({id});
    };

    return {
        create,
        remove,
    }
};

export const useKeyFindByUser = (
    user: UserResponse | undefined, 
    limit: number
): UseInfiniteQueryResult<KeyResponse[], Error> => {
    const {main} = useActors();

    const principal = userGetPrincipal(user);

    return useInfiniteQuery<KeyResponse[], Error>(
        ['keys', principal, limit], 
        ({pageParam = 0}) => keyFindByUser(main, principal, {offset: pageParam, limit}),
        {
            getNextPageParam: (lastPage, pages) => 
                lastPage.length < limit?
                    undefined:
                pages.length * limit
        }
    );
};
