import React from "react";
import { Group, Text } from "@mantine/core";
import { KeyResponse } from "../../../../declarations/main/main.did";

interface Props {
    item: KeyResponse
}

const Item = (props: Props) => {
    const {item} = props;
    
    return (
        <Group key={item.id} position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{item.id}</Text>
                <Text size="xs" color="dimmed">
                    {item.value}
                </Text>
            </div>
        </Group>
    )
};

export default Item;