export interface User {
    uuid: string;
    username: string;
    email: string;
    created_at: string;
    updated_at: string;
}

export interface Document {
    uuid: string;
    title: string;
    content: string;
    user_uuid: string;
    created_at: string;
    updated_at: string;
}

export async function checkAuthentication(): Promise<User | null> {
    try {
        const response = await fetch(`${process.env.VITE_SERVER_URL}/users/me`, {
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

export async function getDocuments(): Promise<Document[]> {
    try {
        const response = await fetch(`${process.env.VITE_SERVER_URL}/documents/all`, {
            method: "GET",
            credentials: "include"
        });
        const data: Document[] = await response.json();
        return data;
    } catch (error) {
        console.warn("Error fetching documents: ", error);
        return [];
    }
}

export async function getDocument(uuid: string): Promise<Document> {
    try {
        const response = await fetch(`${process.env.VITE_SERVER_URL}/documents/${uuid}`, {
            method: "GET",
            credentials: "include"
        });
        const data: Document = await response.json();
        return data;
    } catch (error) {
        console.warn("Error fetching document: ", error);
        return {} as Document;
    }
}
