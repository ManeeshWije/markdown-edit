import React from "react";
import { IconButton, List, ListItem, ListItemPrefix, Input, Drawer, Card } from "@material-tailwind/react";
import { DocumentIcon } from "@heroicons/react/24/solid";
import { MagnifyingGlassIcon, Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import { getDocuments, Document } from "../utils";

export default function Sidebar({ onDocumentClick }: { onDocumentClick: (content: string) => void }) {
    const [isDrawerOpen, setIsDrawerOpen] = React.useState(false);
    const [documents, setDocuments] = React.useState<Document[]>([]);

    React.useEffect(() => {
        const fetchDocuments = async () => {
            const documents = await getDocuments();
            setDocuments(documents);
        };
        fetchDocuments();
    }, []);

    const openDrawer = () => setIsDrawerOpen(true);

    const closeDrawer = () => setIsDrawerOpen(false);

    const getMenuItems = () => {
        return documents.map((doc: Document) => (
            <ListItem onClick={() => handleDocumentClick(doc)} key={doc.uuid} placeholder="list-item">
                <ListItemPrefix placeholder="list-item-prefix">
                    <DocumentIcon className="h-5 w-5" />
                </ListItemPrefix>
                <p className={darkMode ? "text-white" : "text-blue-gray-900"}>{doc.title}</p>
            </ListItem>
        ));
    };

    const handleDocumentClick = (doc: Document) => {
        onDocumentClick(doc.content);
        closeDrawer();
    };

    const darkMode = localStorage.getItem("darkMode") === "true";

    return (
        <>
            <IconButton placeholder="icon-button" variant="text" size="lg" onClick={openDrawer}>
                {isDrawerOpen ? (
                    <XMarkIcon className={darkMode ? "h-8 w-8 stroke-2 text-white" : "h-8 w-8 stroke-2"} />
                ) : (
                    <Bars3Icon className={darkMode ? "h-8 w-8 stroke-2 text-white" : "h-8 w-8 stroke-2"} />
                )}
            </IconButton>
            <Drawer className={darkMode ? "bg-gray-900 text-white" : "bg-white text-blue-gray-900"} placeholder="drawer" open={isDrawerOpen} onClose={closeDrawer} overlay={false}>
                <Card placeholder="card" color={darkMode ? "gray" : "white"} shadow={true} className="h-[calc(100vh-2rem)] w-full p-4">
                    <div className="p-2">
                        <Input placeholder="Search" crossOrigin="true" icon={<MagnifyingGlassIcon className="h-5 w-5" />} label="Search" />
                    </div>
                    <List placeholder="list">
                        <hr className="my-2 border-blue-gray-50" />
                        <p className="align-middle text-center">All Documents</p>
                        {getMenuItems()}
                    </List>
                </Card>
            </Drawer>
        </>
    );
}
