import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router-dom";
import Home from "./pages/Home";
import Editor from "./pages/Editor";
import ErrorPage from "./pages/ErrorPage";
import React, { useState, useEffect } from "react";
import { DefaultSpinner } from "./components/DefaultSpinner";

export interface User {
    uuid: string;
    username: string;
    email: string;
    created_at: string;
    updated_at: string;
}

const App: React.FC = () => {
    const [userData, setUserData] = useState<User | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const checkAuthentication = async () => {
            try {
                const response = await fetch(import.meta.env.VITE_SERVER_URL + "/users/me", {
                    method: "GET",
                    credentials: "include"
                });
                const data: User = await response.json();
                setUserData(data);
            } catch (_) {
                console.warn("You are not authenticated!");
            } finally {
                setLoading(false);
            }
        };

        checkAuthentication();
    }, []);

    if (loading) {
        return <DefaultSpinner />;
    }

    return (
        <Router>
            <Routes>
                <Route path="/" element={<Home userData={userData} />} />
                <Route path="/editor" element={userData ? <Editor /> : <Navigate to="/" />} />
                <Route path="*" element={<ErrorPage />} />
            </Routes>
        </Router>
    );
};

export default App;
