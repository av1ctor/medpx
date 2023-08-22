import React from "react";
import { Button } from "@mantine/core";
import { useCallback } from "react";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { IconUserBolt, IconUserPlus, IconUserX } from "@tabler/icons-react";
import { useBrowser } from "../../../hooks/browser";

interface Props {
    onClick?: () => void;
}

export const AuthMenu = (props: Props) => {
    const {isLogged, logout} = useAuth();
    const {showSuccess} = useUI();
    const {redirectToLogin, redirectToSignup} = useBrowser();
    
    const handleLogout = useCallback(async () => {
        if(props.onClick) {
            props.onClick();
        }
        await logout();
        showSuccess('Logged out!');
    }, [logout, props.onClick]);

    const handleRedirectToLogin = useCallback(() => {
        if(props.onClick) {
            props.onClick();
        }
        redirectToLogin();
    }, [props.onClick]);

    const handleRedirectToSignup = useCallback(() => {
        if(props.onClick) {
            props.onClick();
        }
        redirectToSignup();
    }, [props.onClick]);

    return (!isLogged? 
        <>
            <Button 
                variant="default" 
                leftIcon={<IconUserBolt/>}
                onClick={handleRedirectToLogin}
            >
                Log in
            </Button>
            <Button 
                leftIcon={<IconUserPlus/>}
                onClick={handleRedirectToSignup}
            >
                Sign up
            </Button>
        </>
    :
        <>
            <Button 
                variant="default"
                leftIcon={<IconUserX/>} 
                onClick={handleLogout}
            >
                Log out
            </Button>
        </>
    );
}