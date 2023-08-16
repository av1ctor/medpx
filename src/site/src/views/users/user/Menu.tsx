import React, { useEffect } from "react";
import { Button } from "@mantine/core";
import { useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import { IconUserBolt, IconUserPlus, IconUserX } from "@tabler/icons-react";

export const Menu = () => {
    const {isLogged, logout} = useAuth();
    const {showSuccess} = useUI();
    const navigate = useNavigate();
    
    const redirectToLogin = useCallback(() => {
        navigate(`/user/login?return=${window.location.hash.replace('#', '')}`);
    }, []);

    const redirectToSignup = useCallback(() => {
        navigate(`/user/signup?return=${window.location.hash.replace('#', '')}`);
    }, []);

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