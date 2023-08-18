import React, { Fragment, useCallback } from "react";
import { ActionIcon, Button, Card, Center, Divider, Drawer, Group, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { FormattedMessage } from "react-intl";
import { IconKey, IconPlus, IconRefresh } from "@tabler/icons-react";
import { useAuth } from "../../hooks/auth";
import { useKeyFindByUser } from "../../hooks/keys";
import Item from "./Item";
import KeyCreate from "./key/Create";
import { useUI } from "../../hooks/ui";

interface Props {
}

const Keys = (props: Props) => {
    const {user} = useAuth();
    const {showSuccess} = useUI();
    const [opened, { open, close }] = useDisclosure(false);
    const query = useKeyFindByUser(user, 10);

    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        close();
    }, [close, showSuccess]);

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
                            onClick={open}
                        >
                            <IconPlus size="1rem" />
                        </ActionIcon>
                    </div>
                </Group>

                <Divider pb="xs" />

                {query.status === 'success' && query.data && 
                    query.data.pages.map((page, index) => 
                        <Fragment key={index}>
                            {page.map(item =>
                                <Item key={item.id} item={item} />
                            )}
                        </Fragment>
                    )
                }
                
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
                opened={opened} 
                title={<b><IconKey size="1.25rem" /> Create key</b>}
                position="right"
                size="xl"
                onClose={close} 
            >
                <KeyCreate onSuccess={handleCreated} />
            </Drawer>
        </>
    );
};

export default Keys;