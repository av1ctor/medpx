import React, { useCallback, useState } from "react";
import { ActionIcon, Button, Card, Center, Divider, Drawer, Grid, Group, Modal, Space, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { FormattedMessage } from "react-intl";
import { IconAlertTriangle, IconKey, IconPlus, IconRefresh } from "@tabler/icons-react";
import { useAuth } from "../../hooks/auth";
import { useKey, useKeyFindByUser } from "../../hooks/keys";
import { useUI } from "../../hooks/ui";
import { KeyResponse } from "../../../../declarations/main/main.did";
import Item from "./Item";
import KeyCreate from "./key/Create";

interface Props {
}

const Keys = (props: Props) => {
    const {user} = useAuth();
    const {showSuccess, toggleLoading, showError} = useUI();
    const [createOpened, { open: createOpen, close: createClose }] = useDisclosure(false);
    const [deleteOpened, { open: deleteOpen, close: deleteClose }] = useDisclosure(false);
    const [item, setItem] = useState<KeyResponse|undefined>();
    const {remove} = useKey();
    const query = useKeyFindByUser(user, 10);

    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        createClose();
    }, [createClose, showSuccess]);

    const handleConfirmDeletion = useCallback((item: KeyResponse) => {
        setItem(item);
        deleteOpen()
    }, [setItem, deleteOpen]);

    const handleDelete = useCallback(async () => {
        try {
            toggleLoading(true);
            if(item) {
                await remove(item.id);
            }
            deleteClose()
            showSuccess("Key removed!");
        }
        catch(e) {
            showError(e);
        }
        finally {
            toggleLoading(false);
        }
        
    }, [remove, item]);

    return (
        <>
            <Card withBorder radius="md" p="xl" className="card">
                <Group position="apart" noWrap spacing="xl">
                    <div>
                        <Text fz="lg" className="card-title" fw={500}>
                            <IconKey size="1rem" /> Keys
                        </Text>
                        <Text fz="xs" c="dimmed" mt={3} mb="xl">
                            View your keys
                        </Text>
                    </div>
                    <div>
                        <ActionIcon 
                            variant="filled"
                            color="green"
                            title="New"
                            onClick={createOpen}
                        >
                            <IconPlus size="1rem" />
                        </ActionIcon>
                    </div>
                </Group>

                <Divider pb="xs" />

                {query.status === 'success' && query.data && 
                    query.data.pages.map(page => 
                        page.map(item =>
                            <Item 
                                key={item.id} 
                                item={item} 
                                onDelete={handleConfirmDeletion}
                            />
                        )
                    )
                }

                <Space h="xl" />
                
                <Center>
                    <Button
                        disabled={!query.hasNextPage || query.isFetchingNextPage}
                        onClick={() => query.fetchNextPage()}
                    >
                        <IconRefresh />&nbsp;{<FormattedMessage id={query.hasNextPage? 'Load more': 'All loaded'} defaultMessage={query.hasNextPage? 'Load more': 'All loaded'}/>}
                    </Button>
                </Center>
            </Card>
            
            <Drawer 
                opened={createOpened} 
                title={<b><IconKey size="1.25rem" /> Create key</b>}
                position="right"
                size="xl"
                onClose={createClose} 
            >
                <KeyCreate onSuccess={handleCreated} />
            </Drawer>


            <Modal 
                opened={deleteOpened} 
                title={<b><IconAlertTriangle size="1rem" /> Delete Key</b>}
                centered
                size="xl"
                onClose={deleteClose} 
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
                            color="gray"
                            fullWidth
                            onClick={deleteClose}
                        >
                            Cancel
                        </Button>
                    </Grid.Col>
                </Grid>
            </Modal>
        </>
    );
};

export default Keys;