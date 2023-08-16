import React, { useCallback, useEffect } from "react";
import { Button, Center, Stack, Title } from "@mantine/core";
import { FormattedMessage } from "react-intl";
import { useAuth } from "../../../../../site/src/hooks/auth";
import { useUI } from "../../../../../site/src/hooks/ui";
import { ICProviderType } from "../../../../../site/src/interfaces/icprovider";
import { useBrowser } from "../../../hooks/browser";

interface Props {
}

const Login = (props: Props) => {
    const {login, isLogged} = useAuth();
    const {showError, showSuccess} = useUI();
    const {returnToLastPage} = useBrowser();

    const handleLogin = useCallback(async (providerType: ICProviderType) => {
        try {
            const res = await login(providerType);
            if(res.Err) {
                showError(res.Err);
            }
            else {
                showSuccess('Welcome back!');
                returnToLastPage();
            }
        }
        catch(e) {
            showError(e);
        }
    }, [login]);

    const handleAuthenticateII = useCallback(async () => {
        handleLogin(ICProviderType.InternetIdentity);
    }, [handleLogin]);

    const handleAuthenticatePlug = useCallback(async () => {
        handleLogin(ICProviderType.Plug);
    }, [handleLogin]);
    
    const handleAuthenticateStoic = useCallback(async () => {
        handleLogin(ICProviderType.Stoic);
    }, [handleLogin]);

    useEffect(() => {
        if(isLogged) {
            returnToLastPage();
        }
    }, [isLogged]);

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