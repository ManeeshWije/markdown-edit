import React from "react";
import { IconButton, List, ListItem, ListItemPrefix, Input, Drawer, Card, ListItemSuffix, Alert } from "@material-tailwind/react";
import { DocumentIcon, DocumentPlusIcon, TrashIcon } from "@heroicons/react/24/solid";
import { MagnifyingGlassIcon, Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import { getDocuments, deleteDocument, createDocument, Document } from "../utils";

export default function Sidebar({ onDocumentClick }: { onDocumentClick: (content: string) => void }) {
    const [isDrawerOpen, setIsDrawerOpen] = React.useState(false);
    const [documents, setDocuments] = React.useState<Document[]>([]);
    const [showAlert, setShowAlert] = React.useState(false);

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
                <ListItemSuffix placeholder="list-item-suffix">
                    <IconButton onClick={(e) => handleDelete(e, doc)} variant="outlined" color="red" size="sm" placeholder="delete">
                        <TrashIcon className="h-5 w-5" />
                    </IconButton>
                </ListItemSuffix>
            </ListItem>
        ));
    };

    const handleDocumentClick = (doc: Document) => {
        onDocumentClick(doc.content);
        closeDrawer();
    };

    const handleDelete = (e: React.MouseEvent<HTMLButtonElement>, doc: Document) => {
        e.stopPropagation();
        deleteDocument(doc.uuid);
        setDocuments(documents.filter((d) => d.uuid !== doc.uuid));
        onDocumentClick("");
        doc.content = "";
        onDocumentClick(documents[0].content);
        setShowAlert(true);
    };

    const handleCreate = async () => {
        const newDoc = await createDocument("Untitled");
        setDocuments([...documents, newDoc]);
        onDocumentClick(newDoc.content);
    };

    const darkMode = localStorage.getItem("darkMode") === "true";

    return (
        <>
            {showAlert && (
                <div className="fixed bottom-0 left-0 right-0 flex justify-center items-center p-4 z-50 transform transition-transform duration-300 ease-in-out">
                    <Alert
                        animate={{
                            mount: { y: 0 },
                            unmount: { y: 100 }
                        }}
                        onClose={() => setShowAlert(false)}
                        className="w-60"
                    >
                        Document deleted successfully!
                    </Alert>
                </div>
            )}
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
                    <List placeholder="list-documents">
                        <hr className="my-2 border-blue-gray-50" />
                        <p className="align-middle text-center">All Documents</p>
                        {getMenuItems()}
                    </List>
                    <List placeholder="list-functions" className="bottom-0 absolute">
                        <hr className="my-2 border-blue-gray-50" />
                        <IconButton onClick={handleCreate} variant="text" color="blue" placeholder="create">
                            <DocumentPlusIcon className="h-5 w-5" />
                        </IconButton>
                    </List>
                </Card>
            </Drawer>
        </>
    );
}
