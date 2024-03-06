import { BrowserRouter as Router, Routes, Route, Navigate } from "react-router-dom";
import Home from "./pages/Home";
import Editor from "./pages/Editor";
import ErrorPage from "./pages/ErrorPage";
import React, { useState, useEffect } from "react";
import { DefaultSpinner } from "./components/DefaultSpinner";
import "./input.css";
import { checkAuthentication, User } from "./utils";

const App: React.FC = () => {
    const [userData, setUserData] = useState<User | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        checkAuthentication().then((data) => {
            setUserData(data);
            localStorage.setItem("user_uuid", data?.uuid || "");
            setLoading(false);
        });
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
