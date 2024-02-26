import { Button, Typography } from "@material-tailwind/react";
import { Link } from "react-router-dom";
import Login from "../components/Login";
import Logout from "../components/Logout";
import Footer from "../components/Footer";
import { ReactTyped } from "react-typed";
import "../input.css";

interface User {
    uuid: string;
    username: string;
    email: string;
    created_at: string;
    updated_at: string;
}

const EditorButton = () => {
    return (
        <Link to="/editor">
            <Button color="blue" ripple>
                Go to Editor
            </Button>
        </Link>
    );
};

export default function Home({ userData }: { userData: User | null }) {
    const isDarkMode = localStorage.getItem("darkMode") === "true";
    document.body.classList.toggle("dark-mode", isDarkMode);

    return (
        <div className="flex flex-col justify-center items-center h-screen text-center p-4 relative" id="home">
            {userData && (
                <div className="absolute top-0 right-0 mr-4 mt-4">
                    <Logout />
                </div>
            )}
            <div>
                <Typography variant="h1" className="mt-16 p-4">
                    <ReactTyped backSpeed={50} strings={["Markdown Edit"]} typeSpeed={100} />
                </Typography>

                {userData ? (
                    <>
                        <Typography variant="lead" size="lg" className="p-4">
                            Welcome, {userData.username}!
                        </Typography>
                        <EditorButton />
                    </>
                ) : (
                    <>
                        <Typography variant="h4" color="blue" className="p-4">
                            Please login to continue.
                        </Typography>
                        <Login />
                    </>
                )}
            </div>
            <div className="absolute bottom-0 w-full">
                <Footer />
            </div>
        </div>
    );
}
