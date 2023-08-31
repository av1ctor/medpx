import React from "react";
import { useAuth } from "../../hooks/auth";
import { UserMenu } from "./menus/User";
import { Card } from "@mantine/core";
import Landing from "./Landing";

export const Front = () => {
    const {isLogged} = useAuth();

    if(!isLogged) {
        return <Landing />;
    }

    return (
        <Card withBorder radius="md" p="xl" className="main-card card">
            <UserMenu />
        </Card>
    );
}