import React, { useEffect } from "react";
import { Button, Card, Center, Container, Space, Stack } from "@mantine/core";
import { IconUserBolt } from "@tabler/icons-react";
import { useParams } from "react-router-dom";
import PrescriptionView from "./View";
import { usePrescriptionsFindById } from "../../../hooks/prescriptions";
import { useAuth } from "../../../hooks/auth";
import { useBrowser } from "../../../hooks/browser";
import { useUI } from "../../../hooks/ui";

const PrescriptionViewWrapper = () => {
    const {isLogged} = useAuth();
    const {showError, toggleLoading} = useUI();
    const {redirectToLogin} = useBrowser()
    const {id} = useParams()
    const item = usePrescriptionsFindById(id || '');

    useEffect(() => {
        toggleLoading(item.status === "loading");
        if(item.status === "error") {
            showError(item.error.message);
        }
    }, [item.status]);

    return (
        !isLogged?
            <Container>
                <Center>
                    You must be logged in to be able to access this prescription
                </Center>
                
                <Space />

                <Center>
                    <Stack>
                        <div>
                            Click on the button bellow to log in
                        </div>
                        <div>
                            <Center>
                                <Button 
                                    variant="default" 
                                    leftIcon={<IconUserBolt/>}
                                    onClick={redirectToLogin}
                                >
                                    Log in
                                </Button>
                        </Center>
                        </div>
                    </Stack>
                </Center>
            </Container>
        :
            <Container>
                {item.isLoading || item.data?
                    <Card withBorder>
                        {item.data?
                            <PrescriptionView item={item.data} />
                        :
                            <Center>
                                Please wait, loading...
                                <Space h="30rem" />
                            </Center>
                        }
                    </Card>
                :
                    <Center>
                        Prescription not found or access denied
                    </Center>
                }                
            </Container>
    )
};

export default PrescriptionViewWrapper;