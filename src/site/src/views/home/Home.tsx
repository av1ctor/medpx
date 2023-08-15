import React, { useCallback } from "react";
import { Route, Routes, useLocation, useNavigate } from "react-router-dom";
import { AppShell, Container, useMantineTheme } from '@mantine/core';
import Header from "./Header";
import Footer from "./Footer";
import Login from "../users/user/Login";
import { Front } from "./Front";
import { useUI } from "../../hooks/ui";
import Signup from "../users/user/Signup";

interface Props {
}

const Home = (props: Props) => {
    const theme = useMantineTheme();
    const navigate = useNavigate();
    const location = useLocation();
    const {showSuccess} = useUI();

    const getReturnUrl = (): string => {
        let returnTo = location.search.match(/return=([^&]+)/);
        return (returnTo && returnTo[1]) || '/';
    }

    const handleLogon = useCallback(() => {
        showSuccess("Welcome back!");
        setTimeout(() => handleReturn(), 3000);
    }, []);

    const handleReturn = useCallback(() => {
        navigate(getReturnUrl());
    }, [navigate]);

    return (
        <AppShell
            styles={{
                main: {
                    background: theme.colorScheme === 'dark' ? 
                        theme.colors.dark[8] : 
                        theme.white,
                },
            }}
            navbarOffsetBreakpoint="sm"
            asideOffsetBreakpoint="sm"
            footer={
                <Footer />
            }
            header={
                <Header />
            }
        >
            <Container size="sm">
                <Routes>
                    <Route path="/user/login" element={<Login onLogon={handleLogon} />} />
                    <Route path="/user/signup" element={<Signup onLogon={handleLogon} />} />
                    <Route path="/" element={<Front />} />
                </Routes>
            </Container>
        </AppShell>
    );
};

export default Home;