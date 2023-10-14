import React, { useCallback } from "react";
import { IconClockHour4, IconTrash, IconUser, IconUsersGroup } from "@tabler/icons-react";
import { ActionIcon, Group, Skeleton, Text } from "@mantine/core";
import { Principal } from "@dfinity/principal";
import { PrescriptionAuthResponse } from "../../../../../declarations/main/main.did";
import { prescriptionAuthGetKind } from "../../../libs/prescription_auths";
import TimeFromNow from "../../../components/TimeFromNow";
import { useUserFindById } from "../../../hooks/users";
import { useGroupFindById } from "../../../hooks/groups";

interface Props {
    item: PrescriptionAuthResponse;
    onDelete: (item: PrescriptionAuthResponse) => void;
}

const UserSubject = (
    props: {to: Principal}
) => {
    const user = useUserFindById(props.to);
    return (
        user.data?
            <Text><IconUser size="1rem" /> {user.data.name}</Text>
        :
            <Skeleton h="1rem" w="10rem"></Skeleton>
    );
};

const GroupSubject = (
    props: {to: string}
) => {
    const group = useGroupFindById(props.to);
    return (
        group.data?
            <Text><IconUsersGroup size="1rem" /> {group.data.id}</Text>
        :
            <Skeleton h="1rem" w="10rem"></Skeleton>
    );
};

const Item = (props: Props) => {
    
    const handleDelete = useCallback(async () => {
        props.onDelete(props.item);
    }, [props.item]);

    const {item} = props;

    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                {'User' in item.to && <UserSubject to={item.to.User} />}
                {'Group' in item.to && <GroupSubject to={item.to.Group} />}
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
                <Text size="xs" color="dimmed">
                    {prescriptionAuthGetKind(item.kind).label} /&nbsp;
                    {item.expires_at.length > 0? `Expires at: ${new Date(Number((item.expires_at[0] as bigint) / 1000000n)).toISOString()}`: 'Never expires'}
                </Text>
            </div>
            <ActionIcon
                variant="filled"
                color="red"
                title="Delete"
                onClick={handleDelete}
            >
                <IconTrash size="1rem" />
            </ActionIcon>
        </Group>
    )
};

export default Item;