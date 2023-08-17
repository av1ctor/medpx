import React from "react";
import { useAuth } from "../../hooks/auth";
import Prescriptions from "../prescriptions/Prescriptions";
import Profile from "../users/user/Profile";
import Keys from "../keys/Keys";
import { Space } from "@mantine/core";

export const Front = () => {
    const {isLogged} = useAuth();

    if(!isLogged) {
        return null;
    }

    return (
        <>
            <Prescriptions />
            <Space h="xl" />
            <Keys />
            <Space h="xl" />
            <Profile />
        </>
    );
}