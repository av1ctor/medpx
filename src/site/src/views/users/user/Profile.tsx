import React from "react";
import { Card, Container, Divider, Group, Text } from "@mantine/core";
import { IconUserEdit } from "@tabler/icons-react";
import UserEdit from "./Edit";

const Profile = () => {
    return (
        <Card withBorder radius="md" p="xl" className="card">
            <Group position="apart" noWrap spacing="xl">
                <div>
                    <Text fz="lg" className="card-title" fw={500}>
                        <IconUserEdit size="1rem" /> Profile
                    </Text>
                    <Text fz="xs" c="dimmed" mt={3} mb="xl">
                        Edit your profile
                    </Text>
                </div>
            </Group>

            <Divider pb="md" />

            <Container>
                <UserEdit />
            </Container>
        </Card>
    );
};

export default Profile;