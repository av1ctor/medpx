import React from "react";
import {HashRouter as Router} from "react-router-dom";
import {QueryClient, QueryClientProvider} from 'react-query';
import {AuthContextProvider} from "./stores/auth";
import { ActorContextProvider } from "./stores/actor";
import { IntlContextProvider } from "./stores/intl";
import { UIContextProvider } from "./stores/ui";
import { WalletContextProvider } from "./stores/wallet";
import { IcProviderBuider } from "./libs/icproviderbuilder";
import { MantineProvider } from '@mantine/core';
import { Notifications } from '@mantine/notifications';
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
    return (
        <IntlContextProvider>
            <QueryClientProvider client={queryClient}>
                <AuthContextProvider provider={authProvider}>
                    <WalletContextProvider>
                        <ActorContextProvider>
                            <UIContextProvider>
                                <MantineProvider withGlobalStyles withNormalizeCSS>
                                    <GlobalStyles />
                                    <Notifications position="top-right" />
                                    <Router> 
                                        <Home />
                                    </Router>
                                </MantineProvider>
                            </UIContextProvider>
                        </ActorContextProvider>
                    </WalletContextProvider>
                </AuthContextProvider>
            </QueryClientProvider>
        </IntlContextProvider> 
    );
};


