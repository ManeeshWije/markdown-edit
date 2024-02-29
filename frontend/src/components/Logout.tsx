import { Button } from "@material-tailwind/react";
export default function Logout() {
    localStorage.removeItem("user_uuid");
    localStorage.removeItem("auth-session");
    const authUrl = import.meta.env.VITE_SERVER_URL + "/auth/logout";
    return (
        <div>
            <a href={authUrl}>
                <Button placeholder="Logout" color="gray" ripple>
                    Logout
                </Button>
            </a>
        </div>
    );
}
