import { Button } from "@material-tailwind/react";
export default function Logout() {
    const authUrl = import.meta.env.VITE_SERVER_URL + "/auth/logout";
    return (
        <a href={authUrl}>
            <Button variant="gradient" size="sm" placeholder="Logout">
                Logout
            </Button>
        </a>
    );
}
