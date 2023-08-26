import { UseInfiniteQueryResult, UseQueryResult, useInfiniteQuery, useMutation, useQuery, useQueryClient } from "react-query";
import { GroupRequest, GroupResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { groupCreate, groupDelete, groupFindById, groupFindByUser } from "../libs/groups";
import { userGetPrincipal } from "../libs/users";

interface GroupMethods {
    create: (req: GroupRequest) => Promise<GroupResponse>;
    remove: (id: string) => Promise<void>;
}

export const useGroup = (
): GroupMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: GroupRequest}) => {
            return groupCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['groups']);
            }   
        }
    );

    const create = (
        req: GroupRequest
    ): Promise<GroupResponse> => {
        return createMut.mutateAsync({req});
    };

    const deleteMut = useMutation(
        async (options: {id: string}) => {
            return groupDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['groups']);
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

export const useGroupFindById = (
    id: string
): UseQueryResult<GroupResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<GroupResponse, Error>(
        ['groups', id],
        () => groupFindById(main, id)
    );
}; 

export const useGroupFindByUser = (
    user: UserResponse | undefined, 
    limit: number
): UseInfiniteQueryResult<GroupResponse[], Error> => {
    const {main} = useActors();

    const principal = userGetPrincipal(user);

    return useInfiniteQuery<GroupResponse[], Error>(
        ['groups', principal, limit], 
        ({pageParam = 0}) => groupFindByUser(main, principal, {offset: pageParam, limit}),
        {
            getNextPageParam: (lastPage, pages) => 
                lastPage.length < limit?
                    undefined:
                pages.length * limit
        }
    );
};
