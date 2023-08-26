import React, { forwardRef, useCallback } from "react";
import { ActionIcon, Group } from "@mantine/core";
import { DefaultProps } from '@mantine/styles';
import { IconTrash } from "@tabler/icons-react";
import { useArrayInputContext } from "./Context";

interface ItemProps extends DefaultProps, React.ComponentPropsWithoutRef<'div'> {
    index: number;
    children: any;
}

export const Item = forwardRef<HTMLDivElement, ItemProps>((props: ItemProps, ref) => {
    const ctx = useArrayInputContext();

    const handleDelete = useCallback(() => {
        ctx.onDelete(props.index);
    }, [ctx.onDelete, props.index]);

    return (
        <Group
            ref={ref}    
            position="apart" 
            className="list-item" 
            noWrap 
            spacing="xl"
        >
            <div>
                {props.children}
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
    );
});