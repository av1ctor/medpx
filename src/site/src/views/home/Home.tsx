import React from "react";
import { Route, Routes } from "react-router-dom";
import { AppShell, Container, LoadingOverlay, useMantineTheme } from '@mantine/core';
import Header from "./Header";
import Login from "../users/user/Login";
import { Front } from "./Front";
import { useUI } from "../../hooks/ui";
import Signup from "../users/user/Signup";
import Footer from "./Footer";
import PrescriptionViewWrapper from "../prescriptions/prescription/ViewWrapper";
import Profile from "../users/user/Profile";
import Keys from "../keys/Keys";
import Prescriptions from "../prescriptions/Prescriptions";
import PrescriptionAuths from "../prescriptions/auths/Auths";
import Groups from "../groups/Groups";

interface Props {
}

const Home = (props: Props) => {
    const theme = useMantineTheme();
    const {isLoading} = useUI();

    return (
        <>
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
                header={
                    <Header />
                }
            >
                <Container size="md">
                    <Routes>
                        <Route path="/user/login" element={<Login authenticateOnly={false} />} />
                        <Route path="/user/signup" element={<Signup />} />
                        <Route path="/user/profile" element={<Profile />} />
                        <Route path="/keys" element={<Keys />} />
                        <Route path="/groups" element={<Groups />} />
                        <Route path="/prescriptions" element={<Prescriptions />} />
                        <Route path="/p/:id/auth" element={<PrescriptionAuths />} />
                        <Route path="/p/:id" element={<PrescriptionViewWrapper />} />
                        <Route path="/" element={<Front />} />
                    </Routes>
                </Container>
                
                <Footer />
            </AppShell>
            
            <LoadingOverlay loader={<img src="/loading.svg" />} visible={isLoading}  />
        </>
    );
};

export default Home;