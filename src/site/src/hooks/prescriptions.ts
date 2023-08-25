import { UseInfiniteQueryResult, UseQueryResult, useInfiniteQuery, useMutation, useQuery, useQueryClient } from "react-query";
import { PrescriptionRequest, PrescriptionResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { patientFindPrescriptions } from "../libs/patients";
import { userGetPrincipal, userIsKind } from "../libs/users";
import { doctorFindPrescriptions } from "../libs/doctors";
import { prescriptionCreate, prescriptionDelete, prescriptionFindById } from "../libs/prescriptions";
import { useAuth } from "./auth";
import { thirdPartyFindPrescriptions } from "../libs/thirdparties";

interface PrescriptionMethods {
    create: (req: PrescriptionRequest) => Promise<PrescriptionResponse>;
    remove: (id: string) => Promise<void>;
}

export const usePrescription = (
): PrescriptionMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: PrescriptionRequest}) => {
            return prescriptionCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['prescriptions']);
            }   
        }
    );

    const create = (
        req: PrescriptionRequest
    ): Promise<PrescriptionResponse> => {
        return createMut.mutateAsync({req});
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
        create,
        remove,
    }
};

export const usePrescriptionsFind = (
    user: UserResponse | undefined, 
    limit: number
): UseInfiniteQueryResult<PrescriptionResponse[], Error> => {
    const {main} = useActors();

    const principal = userGetPrincipal(user);

    const fn = userIsKind(user, 'Doctor')? 
        doctorFindPrescriptions
    : 
        userIsKind(user, 'ThirdParty')?
            thirdPartyFindPrescriptions
        :
            patientFindPrescriptions;

    return useInfiniteQuery<PrescriptionResponse[], Error>(
        ['prescriptions', principal, limit], 
        ({pageParam = 0}) => fn(main, principal, {offset: pageParam, limit}),
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
