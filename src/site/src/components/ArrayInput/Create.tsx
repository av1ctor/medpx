import React, { forwardRef, useCallback } from "react";
import { ActionIcon, Box, Button, Center, Container, Grid, Space } from "@mantine/core";
import { DefaultProps } from '@mantine/styles';
import { IconPlus } from "@tabler/icons-react";
import { useArrayInputContext } from "./Context";
import { useDisclosure } from "@mantine/hooks";

interface CreateProps extends DefaultProps, React.ComponentPropsWithoutRef<'div'> {
    children: any;
}

export const Create = forwardRef<HTMLDivElement, CreateProps>((props: CreateProps, ref) => {
    const ctx = useArrayInputContext();
    const [opened, { open, close }] = useDisclosure(false);

    const handleCreate = useCallback(() => {
        close();
        ctx.onCreate();
    }, [ctx.onCreate, close]);

    return (
        <Center>
            {opened &&
                <Container>
                    <Space h="xl"/>
                    {props.children}
                    <Space h="xl"/>
                    <Grid>
                        <Grid.Col md={6}>
                            <Button
                                color="green"
                                fullWidth
                                onClick={handleCreate}
                            >
                                Add
                            </Button>
                        </Grid.Col>
                        <Grid.Col md={6}>
                        <Button
                            color="red"
                            fullWidth
                            onClick={close}
                        >
                            Cancel
                        </Button>
                        </Grid.Col>
                    </Grid>
                </Container>
            }
            <Box display={opened? 'none': 'block'}>
                <ActionIcon 
                    variant="filled"
                    color="green"
                    title="New"
                    onClick={open}
                >
                    <IconPlus size="1rem" />
                </ActionIcon>
            </Box>
        </Center>
    );
});    