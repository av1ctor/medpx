import React from "react";
import { Card, Container, Group, Text } from "@mantine/core";
import UserEdit from "./Edit";

const Profile = () => {
    return (
        <Card withBorder radius="md" p="xl" className="card">
            <Text fz="lg" className="card-title" fw={500}>
                Profile
            </Text>
            <Text fz="xs" c="dimmed" mt={3} mb="xl">
                Edit your profile
            </Text>
            <Container>
                <UserEdit />
            </Container>
        </Card>
    );
};

export default Profile;