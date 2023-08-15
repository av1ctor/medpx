import React, { useEffect, useState } from "react";
import { useCallback } from "react";
import { useAuth } from "../../../hooks/auth";
import { useUI } from "../../../hooks/ui";
import Login from "./Login";
import { Stepper } from "@mantine/core";

interface Props {
    onLogon: () => void;
}

const Signup = (props: Props) => {
    const {login, isAuthenticated, isLogged} = useAuth();
    const {showError, showSuccess} = useUI();
    const [active, setActive] = useState(0);

    const handleLogon = useCallback(() => {
        showSuccess("Welcome back!");
    }, []);

    useEffect(() => {
        if(isLogged) {
            props.onLogon();
        }
        else if(isAuthenticated) {
            if(active === 0) {
                setActive(1);
            }
        }
    }, [isAuthenticated, isLogged, active]);

    return (
        <Stepper 
            active={active} 
            breakpoint="sm"
        >
            <Stepper.Step 
                label="Authentication" 
                description="Authenticate with your provider"
            >
                <Login onLogon={handleLogon} />
            </Stepper.Step>
            <Stepper.Step 
                label="Registration" 
                description="Create a new user"
            >
                Step 2 content
            </Stepper.Step>
        </Stepper>
   );
};

export default Signup;