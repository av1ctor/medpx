import { Button, Center, Container } from "@mantine/core";
import React from "react";
import { useCallback, useState } from "react";
import { FormattedMessage } from "react-intl";
import { useLocation, useNavigate } from "react-router-dom";
import { useAuth } from "../../../../../site/src/hooks/auth";
import { useUI } from "../../../../../site/src/hooks/ui";
import { ICProviderType } from "../../../../../site/src/interfaces/icprovider";

interface Props {
}

const Login = () => {
    const {login} = useAuth();
    const {showSuccess, showError} = useUI();
    const navigate = useNavigate();
    const location = useLocation();

    const getReturnUrl = (): string => {
        let returnTo = location.search.match(/return=([^&]+)/);
        return (returnTo && returnTo[1]) || '/';
    }

    const handleLogin = useCallback(async (providerType: ICProviderType) => {
        try {
            const res = await login(providerType);
            if(res.Err) {
                showError(res.Err);
            }
            else {
                showSuccess("Welcome back!");
                setTimeout(() => handleReturn(), 3000);
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
    }, [login]);

    const handleReturn = useCallback(() => {
        navigate(getReturnUrl());
    }, [navigate]);

    return (
        <Center mx="auto">
            <Container>
                <Center mx="auto">
                    <FormattedMessage defaultMessage="Please choose a provider"/>:
                </Center>
                <Center mx="auto">
                    <Button 
                        onClick={handleAuthenticateII}>
                        <img src="providers/ii.svg" height="" />
                        Internet Identity
                    </Button>
                </Center>
                <br/>
                <Center mx="auto">
                    <Button 
                        color="info"
                        onClick={handleAuthenticatePlug}>
                        <img src="providers/plug.svg" height="" />
                        Plug Wallet
                    </Button>
                </Center>
                <br/>
                <Center mx="auto">
                    <Button 
                        color="warning"
                        onClick={handleAuthenticateStoic}>
                        <img src="providers/stoic.png" height="" />
                        Stoic Wallet
                    </Button>
                </Center>
            </Container>
        </Center>
    );
};

export default Login;