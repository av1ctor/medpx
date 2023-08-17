import React from "react";
import { Group, Text, createStyles, rem } from "@mantine/core";
import { PrescriptionResponse } from "../../../../declarations/main/main.did";

interface Props {
    item: PrescriptionResponse
}

const Item = (props: Props) => {
    const {item} = props;
    
    return (
        <Group key={item.id} position="apart" className="list-item" noWrap spacing="xl">
            <div>
                <Text>{item.id}</Text>
                <Text size="xs" color="dimmed">
                    {item.doctor.toString()}
                </Text>
            </div>
        </Group>
    )
};

export default Item;