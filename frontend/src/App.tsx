import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router-dom";
import Home from "./pages/Home";
import Editor from "./pages/Editor";
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
                setLoading(false); // Set loading to false regardless of success or failure
            }
        };

        checkAuthentication();
    }, []);

    // Render a loading indicator while user data is being fetched
    if (loading) {
        return <DefaultSpinner />;
    }

    // Render the app once user data is fetched
    return (
        <Router>
            <Routes>
                <Route path="/" element={<Home userData={userData} />} />
                <Route path="/editor" element={userData ? <Editor /> : <Navigate to="/" />} />
            </Routes>
        </Router>
    );
};

export default App;
