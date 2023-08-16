import { useCallback } from "react";
import { useMediaQuery } from "@mantine/hooks";
import { useNavigate } from "react-router-dom";

interface BrowserProps {
    isMobile: boolean;
    returnToLastPage: () => void;
    navigateTo: (page: string) => void;
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
    
    return {
        isMobile: useMediaQuery('(max-width: 640px)'),
        returnToLastPage,
        navigateTo,
    };
};
