import React, { useEffect } from "react";
import { Button } from "@mantine/core";
import { useCallback } from "react";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { IconUserBolt, IconUserPlus, IconUserX } from "@tabler/icons-react";
import { useBrowser } from "../../../hooks/browser";

export const Menu = () => {
    const {isLogged, logout} = useAuth();
    const {showSuccess} = useUI();
    const {redirectToLogin, redirectToSignup} = useBrowser();
    
    const handleLogout = useCallback(async () => {
        await logout();
        showSuccess('Logged out!');
    }, [logout]);

    useEffect(() => {

    }, [isLogged]);

    return (!isLogged? 
        <>
            <Button 
                variant="default" 
                leftIcon={<IconUserBolt/>}
                onClick={redirectToLogin}
            >
                Log in
            </Button>
            <Button 
                leftIcon={<IconUserPlus/>}
                onClick={redirectToSignup}
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