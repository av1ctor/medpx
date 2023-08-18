import React, { Fragment, useCallback } from "react";
import { ActionIcon, Button, Card, Center, Divider, Drawer, Group, Space, Text } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { IconClipboardList, IconPlus, IconRefresh } from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";
import { useAuth } from "../../hooks/auth";
import { usePrescriptionsFind } from "../../hooks/prescriptions";
import { useUI } from "../../hooks/ui";
import { userIsKind } from "../../libs/users";
import Item from "./Item";
import PrescriptionCreate from "./prescription/Create";

interface Props {
}

const Prescriptions = (props: Props) => {
    const {user} = useAuth();
    const {showSuccess} = useUI();
    const [opened, { open, close }] = useDisclosure(false);
    const query = usePrescriptionsFind(user, 10);
    
    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        close();
    }, [close, showSuccess]);

    const isDoctor = userIsKind(user, 'Doctor');

    return (
        <>
            <Card withBorder radius="md" p="xl" className="card">
                <Group position="apart" noWrap spacing="xl">
                    <div>
                        <Text fz="lg" className="card-title" fw={500}>
                            <IconClipboardList size="1rem" /> Prescriptions
                        </Text>
                        <Text fz="xs" c="dimmed" mt={3} mb="xl">
                            View your prescriptions
                        </Text>
                    </div>
                    {isDoctor && 
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
                    }
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
                opened={opened} 
                title={<b><IconClipboardList size="1.25rem" /> Create prescription</b>}
                position="right"
                size="xl"
                onClose={close} 
            >
                <PrescriptionCreate onSuccess={handleCreated} />
            </Drawer>
        </>
    );
};

export default Prescriptions;