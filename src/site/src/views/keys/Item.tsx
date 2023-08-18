import React from "react";
import { ActionIcon, Group, Text } from "@mantine/core";
import { KeyResponse } from "../../../../declarations/main/main.did";
import { keyGetKind } from "../../libs/keys";
import { IconTrash } from "@tabler/icons-react";

interface Props {
    item: KeyResponse
}

const Item = (props: Props) => {
    const {item} = props;

    return (
        <Group position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{item.value}</Text>
                <Text size="xs" color="dimmed">
                    {keyGetKind(item.kind).label}
                </Text>
            </div>
            <ActionIcon
                variant="filled"
                color="red"
                title="Delete"
            >
                <IconTrash size="1rem" />
            </ActionIcon>
        </Group>
    )
};

export default Item;