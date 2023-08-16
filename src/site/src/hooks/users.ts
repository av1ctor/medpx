import { UseQueryResult, useQuery } from "react-query";
import { UserResponse } from "../../../declarations/main/main.did";
import { useActors } from "./actors";
import { findMe } from "../libs/users";

export const useFindMe = (
): UseQueryResult<UserResponse, Error> => {
    const {main} = useActors();
    
    return useQuery<UserResponse, Error>(
        ['users', 'me'],
        () => findMe(main)
    );
}; 