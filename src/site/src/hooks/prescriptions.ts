import { UseInfiniteQueryResult, useInfiniteQuery } from "react-query";
import { PrescriptionResponse, UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { patientFindPrescriptions } from "../libs/patients";
import { UserKind, userGetKind, userGetPrincipal } from "../libs/users";
import { doctorFindPrescriptions } from "../libs/doctors";

export const usePrescriptionsFind = (
    user: UserResponse | undefined, 
    limit: number
): UseInfiniteQueryResult<PrescriptionResponse[], Error> => {
    const {main} = useActors();

    const principal = userGetPrincipal(user);
    const isDoctor = userGetKind(user) == UserKind.Doctor;

    return useInfiniteQuery<PrescriptionResponse[], Error>(
        ['prescriptions', principal, limit], 
        ({pageParam = 0}) => isDoctor? 
            doctorFindPrescriptions(main, principal, {offset: pageParam, limit})
        :
            patientFindPrescriptions(main, principal, {offset: pageParam, limit}),
        {
            getNextPageParam: (lastPage, pages) => 
                lastPage.length < limit?
                    undefined:
                pages.length * limit
        }
    );
};
