import { UseInfiniteQueryResult, useInfiniteQuery, useMutation, useQueryClient } from "react-query";
import { KeyRequest, KeyResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { keyCreate, keyFindByUser } from "../libs/keys";
import { userGetPrincipal } from "../libs/users";

interface KeyMethods {
    create: (req: KeyRequest) => Promise<KeyResponse>;
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

    return {
        create,
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
