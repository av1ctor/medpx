import React, { useCallback, useEffect } from "react";
import { Button, Card, Center, Container, Space, Stack } from "@mantine/core";
import { IconUserBolt } from "@tabler/icons-react";
import { useParams } from "react-router-dom";
import PrescriptionView from "./View";
import { usePrescriptionsFindById } from "../../../hooks/prescriptions";
import { useAuth } from "../../../hooks/auth";
import { useBrowser } from "../../../hooks/browser";
import { useUI } from "../../../hooks/ui";
import { PrescriptionResponse } from "../../../../../declarations/main/main.did";

const PrescriptionViewWrapper = () => {
    const {isLogged} = useAuth();
    const {showError, toggleLoading} = useUI();
    const {redirectToLogin, isMobile} = useBrowser()
    const {id} = useParams()
    const item = usePrescriptionsFindById(id || '');

    const Px = useCallback((props: {data: PrescriptionResponse|undefined}) => {
        return props.data?
            <PrescriptionView 
                item={props.data} 
                isEncrypted
            />
        :
            <Center>
                Please wait, loading...
                <Space h="30rem" />
            </Center>
        ;
    }, []);

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
                    !isMobile?
                        <Card withBorder>
                            <Px data={item.data} />
                        </Card>
                    :
                        <Px data={item.data} />
                :
                    <Center>
                        Prescription not found or access denied
                    </Center>
                }                
            </Container>
    )
};

export default PrescriptionViewWrapper;