import { useMutation, useQueryClient } from "react-query";
import { ThirdPartyRequest, ThirdPartyResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { thirdPartyCreate } from "../libs/thirdparties";

interface ThirdPartyMethods {
    create: (req: ThirdPartyRequest) => Promise<ThirdPartyResponse>;
}

export const useThirdParty = (
): ThirdPartyMethods => {
    const queryClient = useQueryClient();
    const {main} = useActors();

    const createMut = useMutation(
        async (options: {req: ThirdPartyRequest}) => {
            return thirdPartyCreate(main, options.req);
        },
        {
            onSuccess: () => {
                queryClient.invalidateQueries(['thirdparties']);
            }   
        }
    );

    const create = (
        req: ThirdPartyRequest
    ): Promise<ThirdPartyResponse> => {
        return createMut.mutateAsync({req});
    };
    
    return {
        create
    }
};