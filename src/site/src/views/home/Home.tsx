import React from "react";
import { Route, Routes } from "react-router-dom";
import { AppShell, useMantineTheme } from '@mantine/core';
import Header from "./Header";
import Footer from "./Footer";
import Login from "../users/user/Login";
import { Front } from "./Front";

interface Props {
}

const Home = (props: Props) => {
    const theme = useMantineTheme();

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
            <>
                <Routes>
                    <Route path="/user/login" element={<Login />} />
                    <Route path="/" element={<Front />} />
                </Routes>
            </>
        </AppShell>
    );
};

export default Home;