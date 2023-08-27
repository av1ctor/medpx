import React, { useCallback, useState } from "react";
import { ActionIcon, Button, Card, Center, Divider, Drawer, Grid, Group, Modal, Space, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { FormattedMessage } from "react-intl";
import { IconAlertTriangle, IconPlus, IconRefresh, IconUsersGroup } from "@tabler/icons-react";
import { useAuth } from "../../hooks/auth";
import { useGroup, useGroupFindByUser } from "../../hooks/groups";
import { useUI } from "../../hooks/ui";
import { GroupResponse } from "../../../../declarations/main/main.did";
import Item from "./Item";
import GroupCreate from "./group/Create";

interface Props {
}

const Groups = (props: Props) => {
    const {user} = useAuth();
    const {showSuccess, toggleLoading, showError} = useUI();
    const [createOpened, { open: createOpen, close: createClose }] = useDisclosure(false);
    const [deleteOpened, { open: deleteOpen, close: deleteClose }] = useDisclosure(false);
    const [item, setItem] = useState<GroupResponse|undefined>();
    const {remove} = useGroup();
    const query = useGroupFindByUser(user, 10);

    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        createClose();
    }, [createClose, showSuccess]);

    const handleConfirmDeletion = useCallback((item: GroupResponse) => {
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
            showSuccess("Group removed!");
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
            <Card withBorder radius="md" p="xl" className="main-card card">
                <Group position="apart" noWrap spacing="xl">
                    <div>
                        <Text fz="lg" className="card-title" fw={500}>
                            <IconUsersGroup size="1rem" /> Groups
                        </Text>
                        <Text fz="xs" c="dimmed" mt={3} mb="xl">
                            View your groups
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
                title={<b><IconUsersGroup size="1.25rem" /> New group</b>}
                position="right"
                size="xl"
                onClose={createClose} 
            >
                <GroupCreate onSuccess={handleCreated} />
            </Drawer>


            <Modal 
                opened={deleteOpened} 
                title={<b><IconAlertTriangle size="1rem" /> Remove Group</b>}
                centered
                size="xl"
                onClose={deleteClose} 
            >
                <Text size="sm" mb="xs" weight={500}>
                    Do you really want to delete this group? <b>This operation can't be reversed!</b>
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

export default Groups;