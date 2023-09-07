import { UseInfiniteQueryResult, UseQueryResult, useInfiniteQuery, useMutation, useQuery, useQueryClient } from "react-query";
import { PrescriptionPreRequest, PrescriptionPostRequest, PrescriptionResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { userFindPrescriptions, userGetPrincipal } from "../libs/users";
import { prescriptionPreCreate, prescriptionPostCreate, prescriptionDelete, prescriptionFindById } from "../libs/prescriptions";
import { useAuth } from "./auth";

interface PrescriptionMethods {
    preCreate: (req: PrescriptionPreRequest) => Promise<PrescriptionResponse>;
    postCreate: (id: string, req: PrescriptionPostRequest) => Promise<PrescriptionResponse>;
    remove: (id: string) => Promise<void>;
}

export const usePrescription = (
): PrescriptionMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const preCreateMut = useMutation(
        async (options: {req: PrescriptionPreRequest}) => {
            return prescriptionPreCreate(main, options.req);
        },
        {
            onSuccess: () => {
            }   
        }
    );

    const preCreate = (
        req: PrescriptionPreRequest
    ): Promise<PrescriptionResponse> => {
        return preCreateMut.mutateAsync({req});
    };

    const postCreateMut = useMutation(
        async (options: {id: string, req: PrescriptionPostRequest}) => {
            return prescriptionPostCreate(main, options.id, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['prescriptions']);
            }   
        }
    );

    const postCreate = (
        id: string,
        req: PrescriptionPostRequest
    ): Promise<PrescriptionResponse> => {
        return postCreateMut.mutateAsync({id, req});
    };

    const deleteMut = useMutation(
        async (options: {id: string}) => {
            return prescriptionDelete(main, options.id);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['prescriptions']);
            }   
        }
    );

    const remove = (
        id: string
    ): Promise<void> => {
        return deleteMut.mutateAsync({id});
    };

    return {
        preCreate,
        postCreate,
        remove,
    }
};

export const usePrescriptionsFind = (
    user: UserResponse | undefined, 
    limit: number
): UseInfiniteQueryResult<PrescriptionResponse[], Error> => {
    const {main} = useActors();

    const principal = userGetPrincipal(user);

    return useInfiniteQuery<PrescriptionResponse[], Error>(
        ['prescriptions', principal, limit], 
        ({pageParam = 0}) => userFindPrescriptions(main, principal, {offset: pageParam, limit}),
        {
            getNextPageParam: (lastPage, pages) => 
                lastPage.length < limit?
                    undefined:
                pages.length * limit
        }
    );
};

export const usePrescriptionsFindById = (
    id: string
): UseQueryResult<PrescriptionResponse, Error> => {
    const {main} = useActors();
    const {isLogged} = useAuth()

    return useQuery<PrescriptionResponse, Error>(
        ['prescriptions', id],
        () => prescriptionFindById(main, id),
        {
            enabled: isLogged
        }
    );
};
