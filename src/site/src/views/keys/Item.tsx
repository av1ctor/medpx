import React, { useCallback } from "react";
import { IconAlertTriangle, IconClockHour4, IconTrash } from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";
import { ActionIcon, Button, Grid, Group, Modal, Space, Text } from "@mantine/core";
import { KeyResponse } from "../../../../declarations/main/main.did";
import { keyGetKind } from "../../libs/keys";
import { useKey } from "../../hooks/keys";
import { useUI } from "../../hooks/ui";
import TimeFromNow from "../../components/TimeFromNow";

interface Props {
    item: KeyResponse
}

const Item = (props: Props) => {
    const [opened, { toggle, close }] = useDisclosure(false);
    const {toggleLoading, showError, showSuccess} = useUI();
    const {remove} = useKey();

    const handleDelete = useCallback(async () => {
        try {
            toggleLoading(true);
            await remove(props.item.id);
            close()
            showSuccess("Key removed!");
        }
        catch(e) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
        
    }, [props.item, remove]);

    const {item} = props;

    return (
        <>
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
                    onClick={toggle}
                >
                    <IconTrash size="1rem" />
                </ActionIcon>
            </Group>

            <Modal 
                opened={opened} 
                title={<b><IconAlertTriangle size="1rem" /> Delete Key</b>}
                centered
                size="xl"
                onClose={close} 
            >
                <Text size="sm" mb="xs" weight={500}>
                    Do you really want to delete this key? <b>This operation can't be reversed!</b>
                </Text>

                <Space h="xl" />

                <Grid>
                    <Grid.Col span={6}>
                        <Button 
                            color="red"
                            fullWidth
                            onClick={handleDelete}
                        >
                            Confirm
                        </Button>
                    </Grid.Col>
                    <Grid.Col span={6}>
                        <Button 
                            color="green"
                            fullWidth
                            onClick={close}
                        >
                            Cancel
                        </Button>
                    </Grid.Col>
                </Grid>
            </Modal>
        </>
    )
};

export default Item;