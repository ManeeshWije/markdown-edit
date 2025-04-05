import { Button } from "@material-tailwind/react";
export default function Logout() {
    const serverUrl = import.meta.env.MODE === "production" ? "" : "http://localhost:8080/auth/logout"
    return (
        <a href={serverUrl}>
            <Button variant="gradient" size="sm" placeholder="Logout">
                Logout
            </Button>
        </a>
    );
}
