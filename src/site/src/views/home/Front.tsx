import React from "react";
import { useAuth } from "../../hooks/auth";
import { UserMenu } from "./menus/User";
import { Card, SimpleGrid } from "@mantine/core";

export const Front = () => {
    const {isLogged} = useAuth();

    if(!isLogged) {
        return null;
    }

    return (
        <Card withBorder radius="md" p="xl" className="main-card card">
            <SimpleGrid cols={2} spacing={0}>
                <UserMenu />
            </SimpleGrid>
        </Card>
    );
}