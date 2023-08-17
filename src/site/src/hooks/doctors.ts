import { useMutation, useQueryClient } from "react-query";
import { DoctorRequest, DoctorResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { doctorCreate } from "../libs/doctors";

interface DoctorMethods {
    create: (req: DoctorRequest) => Promise<DoctorResponse>;
}

export const useDoctor = (
): DoctorMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: DoctorRequest}) => {
            return doctorCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['doctors']);
            }   
        }
    );

    const create = (
        req: DoctorRequest
    ): Promise<DoctorResponse> => {
        return createMut.mutateAsync({req});
    };
    
    return {
        create
    }
};