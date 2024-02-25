import { Button, Typography } from "@material-tailwind/react";
import { Link } from "react-router-dom";
import Login from "../components/Login";
import Logout from "../components/Logout";

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
    return (
        <div>
            <Typography color="blue" size="xl">
                Home
            </Typography>
            {userData && (
                <div>
                    <Typography color="blue" size="md">
                        Welcome, {userData.username}!
                    </Typography>
                    <EditorButton />
                </div>
            )}
            {!userData && <Login />}
            {userData && <Logout />}
        </div>
    );
}
