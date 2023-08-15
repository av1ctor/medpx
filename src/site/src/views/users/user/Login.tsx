import { Button, Center, Stack, Title } from "@mantine/core";
import React from "react";
import { useCallback } from "react";
import { FormattedMessage } from "react-intl";
import { useAuth } from "../../../../../site/src/hooks/auth";
import { useUI } from "../../../../../site/src/hooks/ui";
import { ICProviderType } from "../../../../../site/src/interfaces/icprovider";

interface Props {
    onLogon: () => void;
}

const Login = (props: Props) => {
    const {login, isAuthenticated} = useAuth();
    const {showError} = useUI();

    const handleLogin = useCallback(async (providerType: ICProviderType) => {
        try {
            const res = await login(providerType);
            if(res.Err) {
                showError(res.Err);
            }
            else {
                props.onLogon();
            }
        }
        catch(e) {
            showError(e);
        }
    }, [login, props.onLogon]);

    const handleAuthenticateII = useCallback(async () => {
        handleLogin(ICProviderType.InternetIdentity);
    }, [handleLogin]);

    const handleAuthenticatePlug = useCallback(async () => {
        handleLogin(ICProviderType.Plug);
    }, [handleLogin]);
    
    const handleAuthenticateStoic = useCallback(async () => {
        handleLogin(ICProviderType.Stoic);
    }, [handleLogin]);

    if(isAuthenticated) {
        props.onLogon();
        return null;
    }

    return (
        <Stack>
            <Center>
                <Title order={4}>
                    <FormattedMessage defaultMessage="Please choose a provider"/>
                </Title>
            </Center>
            <Center>
                <Button 
                    color="blue"
                    leftIcon={<img src="providers/ii.svg" height="24" />}
                    onClick={handleAuthenticateII}
                >
                    Internet Identity
                </Button>
            </Center>
            <Center>
                <Button 
                    color="pink"
                    leftIcon={<img src="providers/plug.svg" />}
                    onClick={handleAuthenticatePlug}
                >
                    Plug Wallet
                </Button>
            </Center>
            <Center>
                <Button 
                    color="cyan"
                    leftIcon={<img src="providers/stoic.png" height="24" />}
                    onClick={handleAuthenticateStoic}
                >
                    Stoic Wallet
                </Button>
            </Center>
        </Stack>
    );
};

export default Login;