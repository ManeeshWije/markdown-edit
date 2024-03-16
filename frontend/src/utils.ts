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
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/users/me`, {
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
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/documents/all`, {
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
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/documents/${uuid}`, {
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

export async function createDocument(title: string): Promise<Document> {
    try {
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/documents/create`, {
            method: "POST",
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                uuid: crypto.randomUUID(),
                title,
                content: "",
                user_uuid: localStorage.getItem("user_uuid"),
                created_at: new Date().toISOString(),
                updated_at: new Date().toISOString()
            })
        });
        const data: Document = await response.json();
        return data;
    } catch (error) {
        console.warn("Error creating document: ", error);
        return {} as Document;
    }
}

export async function deleteDocument(uuid: string): Promise<void> {
    try {
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/documents/delete/${uuid}`, {
            method: "DELETE",
            credentials: "include"
        });
        const data = await response.json();
        return data;
    } catch (error) {
        console.warn("Error deleting document: ", error);
    }
}

export async function updateDocument(uuid: string, title: string, content: string): Promise<Document> {
    try {
        const response = await fetch(`${import.meta.env.VITE_SERVER_URL}/documents/update/${uuid}`, {
            method: "PUT",
            credentials: "include",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                uuid,
                title,
                content,
                updated_at: new Date().toISOString()
            })
        });
        const data: Document = await response.json();
        return data;
    } catch (error) {
        console.warn("Error updating document: ", error);
        return {} as Document;
    }
}

export async function exportToHTML(filename: string, markdownContent: string) {
    const url = "https://api.github.com/markdown";
    const options = {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            text: markdownContent,
            mode: "markdown"
        })
    };
    const response = await fetch(url, options);
    const html = await response.text();
    const styledHTML = `
            <html>
            <head>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                    }
                </style>
            </head>
            <body>${html}</body>
            </html>
            `;
    const blob = new Blob([styledHTML], { type: "text/html" });
    const objURL = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = objURL;
    a.download = `${filename}.html`;
    a.click();
}
