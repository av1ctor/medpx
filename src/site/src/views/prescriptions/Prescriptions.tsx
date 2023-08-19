import React, { Fragment, useCallback, useState } from "react";
import { ActionIcon, Button, Card, Center, Divider, Drawer, Group, Modal, Space, Text } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { IconClipboardList, IconPlus, IconRefresh } from "@tabler/icons-react";
import { useDisclosure } from "@mantine/hooks";
import { useAuth } from "../../hooks/auth";
import { usePrescriptionsFind } from "../../hooks/prescriptions";
import { useUI } from "../../hooks/ui";
import { userIsKind } from "../../libs/users";
import { PrescriptionResponse } from "../../../../declarations/main/main.did";
import Item from "./Item";
import PrescriptionCreate from "./prescription/Create";
import PrescriptionView from "./prescription/View";
import { useBrowser } from "../../hooks/browser";

interface Props {
}

const Prescriptions = (props: Props) => {
    const {user} = useAuth();
    const {showSuccess} = useUI();
    const {isMobile} = useBrowser()
    const [createOpened, { open: createOpen, close: createClose }] = useDisclosure(false);
    const [viewOpened, { open: viewOpen, close: viewClose }] = useDisclosure(false);
    const [item, setItem] = useState<PrescriptionResponse|undefined>();
    const query = usePrescriptionsFind(user, 10);
    
    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        createClose();
    }, [createClose, showSuccess]);

    const handleView = useCallback((item: PrescriptionResponse) => {
        setItem(item);
        viewOpen()
    }, [setItem, viewOpen]);

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
                                onClick={createOpen}
                            >
                                <IconPlus size="1rem" />
                            </ActionIcon>
                        </div>
                    }
                </Group>
                
                <Divider pb="xs" />

                {query.status === 'success' && query.data && 
                    query.data.pages.map(page => 
                        page.map(item =>
                            <Item 
                                key={item.id} 
                                item={item} 
                                onView={handleView}
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
                title={<b><IconClipboardList size="1.25rem" /> Create prescription</b>}
                position="right"
                size="xl"
                onClose={createClose} 
            >
                <PrescriptionCreate onSuccess={handleCreated} />
            </Drawer>
            
            <Modal
                opened={viewOpened}
                size="xl"
                fullScreen={isMobile}
                centered
                onClose={viewClose}
            >
                {item && <PrescriptionView 
                    item={item} 
                />}
            </Modal>
        </>
    );
};

export default Prescriptions;