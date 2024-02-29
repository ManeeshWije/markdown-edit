import { Button, Typography } from "@material-tailwind/react";
import { Link } from "react-router-dom";
import Login from "../components/Login";
import Logout from "../components/Logout";
import Footer from "../components/Footer";
import { ReactTyped } from "react-typed";
import "../input.css";
import { User } from "../utils";

const EditorButton = () => {
    return (
        <Link to="/editor">
            <Button placeholder="Go To Editor" color="blue" ripple>
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
                <Typography placeholder="markdown edit" variant="h1" className="mt-16 p-4">
                    <ReactTyped backSpeed={50} strings={["Markdown Edit"]} typeSpeed={100} />
                </Typography>

                {userData ? (
                    <>
                        <Typography placeholder="Welcome, <username>" variant="lead" className="p-4">
                            Welcome, {userData.username}!
                        </Typography>
                        <EditorButton />
                    </>
                ) : (
                    <>
                        <Typography placeholder="Please login to continue." variant="h4" color="blue" className="p-4">
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
