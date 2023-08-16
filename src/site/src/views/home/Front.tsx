import React from "react";
import { useAuth } from "../../hooks/auth";

export const Front = () => {
    const {isLogged, user} = useAuth();

    return (
        <>
            {isLogged && <>
                    {JSON.stringify(user, (_, v) => typeof v === 'bigint' ? v.toString() : v, 4)}
            </>}
        </>
    );
}