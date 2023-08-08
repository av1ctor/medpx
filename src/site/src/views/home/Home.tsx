import React from "react";
import {Routes, Route} from "react-router-dom";
import { useUI } from "../../hooks/ui";
import Front from "./Front";

export const Home = () => {
    const {isLoading} = useUI();

    return (
        <div className="home">
            <section className="section">
                <div className="container">
                    <Routes>
                        <Route path="/" element={<Front />} />
                    </Routes>
                </div>
            </section>

            {/*<div className={`loading ${isLoading? 'visible': 'hidden'}`}>
                <img src="/loading.svg" />
            </div>*/}
        </div>            
    );
}