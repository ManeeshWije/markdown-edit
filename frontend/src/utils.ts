export interface User {
    uuid: string;
    username: string;
    email: string;
    created_at: string;
    updated_at: string;
}

export async function checkAuthentication(): Promise<User | null> {
    try {
        const response = await fetch(import.meta.env.VITE_SERVER_URL + "/users/me", {
            method: "GET",
            credentials: "include"
        });
        const data: User = await response.json();
        return data;
    } catch (_) {
        console.warn("You are not authenticated!");
        return null;
    }
}
