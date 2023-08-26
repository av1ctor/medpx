import React, { useCallback, useState } from "react";
import { ActionIcon, Button, Card, Divider, Drawer, Grid, Group, Modal, Space, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconAlertTriangle, IconPlus, IconShare } from "@tabler/icons-react";
import { useParams } from "react-router-dom";
import { usePrescriptionAuth, usePrescriptionAuthsFindByPrescription } from "../../../hooks/prescription_auths";
import { useUI } from "../../../hooks/ui";
import { PrescriptionAuthResponse } from "../../../../../declarations/main/main.did";
import Item from "./Item";
import PrescriptionAuthCreate from "./auth/Create";

interface Props {
}

const PrescriptionAuths = (props: Props) => {
    const {showSuccess, toggleLoading, showError} = useUI();
    const [createOpened, { open: createOpen, close: createClose }] = useDisclosure(false);
    const [deleteOpened, { open: deleteOpen, close: deleteClose }] = useDisclosure(false);
    const [item, setItem] = useState<PrescriptionAuthResponse|undefined>();
    const {remove} = usePrescriptionAuth();
    const {id} = useParams();
    const query = usePrescriptionAuthsFindByPrescription(id);

    const handleCreated = useCallback((msg: string) => {
        showSuccess(msg);
        createClose();
    }, [createClose, showSuccess]);

    const handleConfirmDeletion = useCallback((item: PrescriptionAuthResponse) => {
        setItem(item);
        deleteOpen()
    }, [setItem, deleteOpen]);

    const handleDelete = useCallback(async () => {
        try {
            toggleLoading(true);
            if(item) {
                await remove(item);
            }
            deleteClose()
            showSuccess("Prescription authorization removed!");
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
                            <IconShare size="1rem" /> Authorizations
                        </Text>
                        <Text fz="xs" c="dimmed" mt={3} mb="xl">
                            See who can access this prescription
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
                    query.data.map(item => 
                        <Item 
                            key={item.id} 
                            item={item} 
                            onDelete={handleConfirmDeletion}
                        />
                    )
                }
            </Card>
            
            <Drawer 
                opened={createOpened} 
                title={<b><IconShare size="1.25rem" /> New authorization</b>}
                position="right"
                size="xl"
                onClose={createClose} 
            >
                {id &&
                    <PrescriptionAuthCreate 
                        prescriptionId={id}
                        onSuccess={handleCreated} 
                    />
                }
            </Drawer>

            <Modal 
                opened={deleteOpened} 
                title={<b><IconAlertTriangle size="1rem" /> Remove authorization</b>}
                centered
                size="xl"
                onClose={deleteClose} 
            >
                <Text size="sm" mb="xs" weight={500}>
                    Do you really want to delete this authorization? <b>This operation can't be reversed!</b>
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

export default PrescriptionAuths;