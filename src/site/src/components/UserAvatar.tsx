import { Avatar, Badge, Group, Skeleton, Space, Text } from "@mantine/core";
import React from "react";
import { UserResponse } from "../../../declarations/main/main.did";
import { principalToString } from "../libs/icp";
import { Link } from "react-router-dom";

interface Props {
    user: UserResponse|undefined
}

export const UserAvatar = (
    props: Props
) => {
    
    const {user} = props;
    
    return (
        <Group noWrap spacing="sm">
            {user?
                <>
                    <Avatar color="cyan" radius="xl">
                        {user.name.substr(0, 1).toUpperCase()}
                    </Avatar>
                    <div>
                        <div>
                            <Badge size="xs">
                                <Link target="blank" to={`/user/${user.id}`}>
                                    {principalToString(user.id)}
                                </Link>
                            </Badge>
                        </div>
                        <div>
                            <Text fw={500}>{user.name}</Text>
                        </div>
                    </div>
                </>
            :
                <>
                    <Skeleton height={40} circle />
                    <div>
                        <Skeleton width="8rem" h="1rem" />
                        <Space h="sm" />
                        <Skeleton width="8rem" h="1rem" />
                    </div>                    
                </>
            }
        </Group>
    );
};