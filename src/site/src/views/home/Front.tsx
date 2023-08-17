import React from "react";
import { useAuth } from "../../hooks/auth";
import Prescriptions from "../prescriptions/Prescriptions";
import Profile from "../users/user/Profile";

export const Front = () => {
    const {isLogged} = useAuth();

    if(!isLogged) {
        return null;
    }

    return (
        <>
            <Prescriptions />
            <Profile />
        </>
    );
}