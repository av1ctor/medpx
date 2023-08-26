import React, { useCallback } from "react";
import { IconClockHour4, IconTrash } from "@tabler/icons-react";
import { ActionIcon, Badge, Group, Text } from "@mantine/core";
import { GroupResponse } from "../../../../declarations/main/main.did";
import TimeFromNow from "../../components/TimeFromNow";
import { principalToString } from "../../libs/icp";

interface Props {
    item: GroupResponse;
    onDelete: (item: GroupResponse) => void;
}

const Item = (props: Props) => {
    const handleDelete = useCallback(async () => {
        props.onDelete(props.item);
    }, [props.item]);

    const {item} = props;

    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>
                    Members: {item.members.map(m => 
                        <Badge key={m.toString()}>{principalToString(m)}</Badge>)
                    }
                </Text>
                <Text size="xs"><IconClockHour4 size="0.75rem"/> <TimeFromNow date={item.created_at} /></Text>
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