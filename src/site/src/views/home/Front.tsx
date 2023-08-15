import React from "react";
import { useUI } from "../../hooks/ui";
import { useAuth } from "../../hooks/auth";
import { Button } from "@mantine/core";

export const Front = () => {
    const {isLoading} = useUI();
    const {isLogged} = useAuth();

    return (
        <>
            {!isLogged && <>
                    <Button>Click here to login or create an account</Button>
            </>}
            {/*<div className={`loading ${isLoading? 'visible': 'hidden'}`}>
                <img src="/loading.svg" />
            </div>*/}
        </>
    );
}