import React, { useCallback } from "react";
import {HashRouter as Router} from "react-router-dom";
import {QueryClient, QueryClientProvider} from 'react-query';
import {AuthContextProvider} from "./stores/auth";
import { ActorContextProvider } from "./stores/actor";
import { IntlContextProvider } from "./stores/intl";
import { UIContextProvider } from "./stores/ui";
import { WalletContextProvider } from "./stores/wallet";
import { IcProviderBuider } from "./libs/icproviderbuilder";
import { ColorScheme, ColorSchemeProvider, MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
import { useLocalStorage } from "@mantine/hooks";
import Home from "./views/home/Home";
import { GlobalStyles } from "./GlobaStyles";

const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            staleTime: Infinity,
        },
    },
});

const authProvider = new IcProviderBuider()
    .withInternetIdentity()
    .withPlug()
    .build();

export const App = () => {
    const [colorScheme, setColorScheme] = useLocalStorage<ColorScheme>({
        key: 'mantine-color-scheme',
        defaultValue: 'light',
        getInitialValueInEffect: true,
    });
    
    const toggleColorScheme = useCallback((value?: ColorScheme) => {
        setColorScheme(value || (colorScheme === 'dark' ? 'light' : 'dark'));
    }, [colorScheme, setColorScheme]);
        
    return (
        <IntlContextProvider>
            <QueryClientProvider client={queryClient}>
                <AuthContextProvider provider={authProvider}>
                    <WalletContextProvider>
                        <ActorContextProvider>
                            <UIContextProvider>
                                <ColorSchemeProvider colorScheme={colorScheme} toggleColorScheme={toggleColorScheme}>
                                    <MantineProvider theme={{ colorScheme }} withGlobalStyles withNormalizeCSS>
                                        <GlobalStyles />
                                        <Notifications position="bottom-right" />
                                        <Router> 
                                            <Home />
                                        </Router>
                                    </MantineProvider>
                                </ColorSchemeProvider>
                            </UIContextProvider>
                        </ActorContextProvider>
                    </WalletContextProvider>
                </AuthContextProvider>
            </QueryClientProvider>
        </IntlContextProvider> 
    );
};


