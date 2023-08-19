import { useCallback } from "react";
import { useMediaQuery } from "@mantine/hooks";
import { useNavigate } from "react-router-dom";

interface BrowserProps {
    isMobile: boolean;
    returnToLastPage: () => void;
    navigateTo: (page: string) => void;
    redirectToLogin: () => void;
    redirectToSignup: () => void;
};

export const useBrowser = (): BrowserProps => {
    const navigate = useNavigate();
    
    const getReturnUrl = (): string => {
        let returnTo = location.search.match(/return=([^&]+)/);
        return (returnTo && returnTo[1]) || '/';
    }

    const returnToLastPage = useCallback(() => {
        navigate(getReturnUrl());
    }, []);
    
    const navigateTo = useCallback((page: string) => {
        navigate(page);
    }, []);

    const redirectToLogin = useCallback(() => {
        navigate(`/user/login?return=${window.location.hash.replace('#', '')}`);
    }, []);

    const redirectToSignup = useCallback(() => {
        navigate(`/user/signup?return=${window.location.hash.replace('#', '')}`);
    }, []);
    
    return {
        isMobile: !useMediaQuery('(min-width: 62em)'),
        returnToLastPage,
        navigateTo,
        redirectToLogin,
        redirectToSignup,
    };
};
