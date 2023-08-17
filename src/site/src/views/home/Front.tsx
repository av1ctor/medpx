import React from "react";
import { useAuth } from "../../hooks/auth";
import Prescriptions from "../prescriptions/Prescriptions";
import UserEdit from "../users/user/Edit";
import Profile from "../users/user/Profile";

export const Front = () => {
    const {isLogged, user} = useAuth();

    return (
        <>
            {isLogged && <>
                <Prescriptions />
                <Profile />
            </>}
        </>
    );
}