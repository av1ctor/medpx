import React, { useCallback } from "react";
import { IconClockHour4, IconTrash } from "@tabler/icons-react";
import { ActionIcon, Group, Text } from "@mantine/core";
import { KeyResponse } from "../../../../declarations/main/main.did";
import { keyGetKind } from "../../libs/keys";
import TimeFromNow from "../../components/TimeFromNow";

interface Props {
    item: KeyResponse;
    onDelete: (item: KeyResponse) => void;
}

const Item = (props: Props) => {
    const handleDelete = useCallback(async () => {
        props.onDelete(props.item);
    }, [props.item]);

    const {item} = props;

    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{item.value}</Text>
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
                <Text size="xs" color="dimmed">
                    {keyGetKind(item.kind).label}
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