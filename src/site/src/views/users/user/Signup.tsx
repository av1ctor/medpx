import React, { useEffect, useMemo, useState, useCallback } from "react";
import { Box, Center, Container, SegmentedControl, Space, Stepper } from "@mantine/core";
import { IconStethoscope, IconUserHeart, IconUsersGroup } from "@tabler/icons-react";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { useBrowser } from "../../../hooks/browser";
import Login from "./Login";
import PatientCreate from "../../patients/patient/Create";

interface Props {
}

const Signup = () => {
    const {isAuthenticated, isLogged} = useAuth();
    const {showSuccess} = useUI();
    const {isMobile, returnToLastPage} = useBrowser();
    const [active, setActive] = useState(0);
    const [kind, setKind] = useState('patient');

    const handleChangeKind = useCallback((value: string) => {
        setKind(value);
    }, [setKind]);

    const handleRegistered = useCallback((msg: string) => {
        showSuccess(msg);
        returnToLastPage();
    }, []);

    useEffect(() => {
        if(isLogged) {
            returnToLastPage();
        }
        else if(isAuthenticated) {
            if(active === 0) {
                setActive(1);
            }
        }
    }, [isAuthenticated, isLogged, active, setActive]);

    const kinds = useMemo(() => {
        return [
            {
                value: 'patient',
                label: (
                    <Center>
                        <IconUserHeart size="1rem" />
                        <Box ml={10}>Patient</Box>
                    </Center>
                ),
            },
            {
                value: 'doctor',
                label: (
                    <Center>
                        <IconStethoscope size="1rem" />
                        <Box ml={10}>Doctor</Box>
                    </Center>
                ),
            },
            {
                value: 'thirdparty',
                label: (
                    <Center>
                        <IconUsersGroup size="1rem" />
                        <Box ml={10}>Third party</Box>
                    </Center>
                ),
            },
        ];
    }, []);

    return (
        <Stepper 
            active={active} 
            breakpoint="sm"
            color="green"
        >
            <Stepper.Step 
                label="Authentication" 
                description="Authenticate with your provider"
            >
                <Login />
            </Stepper.Step>
            <Stepper.Step 
                label="Registration" 
                description="Create a new user"
            >
                <Space h="md" />
                <Container>
                    <SegmentedControl
                        size="md"
                        orientation={isMobile? "vertical": "horizontal"}
                        fullWidth
                        color="blue"
                        value={kind}
                        data={kinds}
                        onChange={handleChangeKind}
                    />
                </Container>
                <Space h="md" />
                {kind === 'patient' && 
                    <PatientCreate onSuccess={handleRegistered} />
                }
            </Stepper.Step>
        </Stepper>
   );
};

export default Signup;