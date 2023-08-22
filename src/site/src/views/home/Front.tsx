import React, { useMemo } from "react";
import { useAuth } from "../../hooks/auth";
import { UserMenu } from "./menus/User";
import { Card, SimpleGrid } from "@mantine/core";

export const Front = () => {
    const {isLogged} = useAuth();

    const menu = useMemo(() => {
        return <UserMenu />;
    }, []);

    if(!isLogged) {
        return null;
    }

    return (
        <Card withBorder radius="md" p="xl" className="card">
            <SimpleGrid cols={2} spacing={0}>
                <UserMenu />
            </SimpleGrid>
        </Card>
    );
}