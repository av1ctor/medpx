import React from "react";
import { Button, createStyles } from "@mantine/core";
import { useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";

const useStyles = createStyles((theme) => ({
    hiddenMobile: {
        [theme.fn.smallerThan('sm')]: {
          display: 'none',
        },
      },
    
      hiddenDesktop: {
        [theme.fn.largerThan('sm')]: {
          display: 'none',
        },
      },
}));

export const Menu = () => {
    const {isLogged, user, logout} = useAuth();
    const {showSuccess} = useUI();
    const navigate = useNavigate();
    const {classes} = useStyles();
    
    const redirectToLogin = useCallback(() => {
        navigate(`/user/login?return=${window.location.hash.replace('#', '')}`);
    }, []);

    const handleLogout = useCallback(async () => {
        await logout();
        showSuccess('Logged out!');
    }, [logout]);

    return (!isLogged? 
        <>
            <Button 
                variant="default" 
                onClick={redirectToLogin}
            >
                Log in
            </Button>
            <Button>
                Sign up
            </Button>
        </>
    :
        <>
            <Button 
                variant="default" 
                onClick={handleLogout}
            >
                Log out
            </Button>
        </>
    );
}