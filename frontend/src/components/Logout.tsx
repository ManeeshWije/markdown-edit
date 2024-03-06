import { Button } from "@material-tailwind/react";
export default function Logout() {
    const authUrl = import.meta.env.VITE_SERVER_URL + "/auth/logout";
    return (
        <div>
            <a href={authUrl}>
                <Button variant="gradient" size="sm" className="lg:inline-block" placeholder="Logout" color="gray" ripple>
                    Logout
                </Button>
            </a>
        </div>
    );
}
