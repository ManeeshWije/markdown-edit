import React from "react";
import {
    IconButton,
    List,
    ListItem,
    ListItemPrefix,
    Input,
    Drawer,
    Card,
    ListItemSuffix,
    Alert,
    Dialog,
    DialogHeader,
    DialogBody,
    DialogFooter,
    Button
} from "@material-tailwind/react";
import { DocumentIcon, DocumentPlusIcon, TrashIcon, PencilSquareIcon } from "@heroicons/react/24/solid";
import { MagnifyingGlassIcon, Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import { getDocuments, getDocument, deleteDocument, Document, createDocument, updateDocument } from "../utils";
import { useStore } from "../store";

export default function Sidebar({ onDocumentClick }: { onDocumentClick: (content: string) => void }) {
    const { documents, selectedDoc, setDocuments, setSelectedDoc } = useStore();
    const [isDrawerOpen, setIsDrawerOpen] = React.useState(false);
    const [showAlert, setShowAlert] = React.useState(false);
    const [open, setOpen] = React.useState(false);
    const [newDoc, setNewDoc] = React.useState("");
    const [action, setAction] = React.useState("");

    React.useEffect(() => {
        const fetchDocuments = async () => {
            const documents = await getDocuments();
            setDocuments(documents);
        };
        fetchDocuments();
    }, [setDocuments]);

    // show alert for 3 seconds when a document is deleted
    React.useEffect(() => {
        if (showAlert) {
            const timer = setTimeout(() => {
                setShowAlert(false);
            }, 3000);
            return () => clearTimeout(timer);
        }
    }, [showAlert]);

    const openDrawer = () => setIsDrawerOpen(true);

    const closeDrawer = () => setIsDrawerOpen(false);

    const getMenuItems = () => {
        documents.sort((a, b) => (a.updated_at > b.updated_at ? -1 : 1));
        return documents.map((doc: Document) => (
            <ListItem onClick={() => handleDocumentClick(doc)} key={doc.uuid} placeholder="list-item" id={`${doc.uuid}`} className="bruh-item">
                <ListItemPrefix placeholder="list-item-prefix" className={selectedDoc.uuid === doc.uuid ? "text-blue-500" : "text-blue-gray-500"}>
                    <DocumentIcon className="h-5 w-5" />
                </ListItemPrefix>
                <p className={darkMode ? "text-white" : "text-blue-gray-900"}>{doc.title}</p>
                <ListItemSuffix className="flex gap-2" placeholder="list-item-suffix">
                    <IconButton onClick={(e) => handleDelete(e, doc)} variant="outlined" color="red" size="sm" placeholder="delete">
                        <TrashIcon className="h-5 w-5" />
                    </IconButton>
                    <IconButton onClick={(e) => handleOpenDialog(e, "update", doc)} variant="outlined" color="blue" size="sm" placeholder="update">
                        <PencilSquareIcon className="h-5 w-5" />
                    </IconButton>
                </ListItemSuffix>
            </ListItem>
        ));
    };

    const handleDocumentClick = async (doc: Document) => {
        const document = await getDocument(doc.uuid);
        onDocumentClick(document.content);
        setSelectedDoc(document);
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

    // based on the action, create or update the document
    // combined into one as they are so similar
    const handleCreateOrUpdate = async (action: string, title: string, doc: Document) => {
        openDrawer();
        if (action === "create") {
            const newDoc = await createDocument(title);
            setDocuments([...documents, newDoc]);
            setSelectedDoc(newDoc);
        } else {
            const updatedDoc = await updateDocument(doc.uuid, title, doc.content);
            const updatedDocs = documents.map((d) => (d.uuid === updatedDoc.uuid ? updatedDoc : d));
            setDocuments(updatedDocs);
        }
        setOpen(false);
        setAction("");
    };

    const handleOpenDialog = (e: React.MouseEvent<HTMLButtonElement>, action: string, doc: Document) => {
        e.preventDefault();
        e.stopPropagation();
        if (action === "update") {
            setNewDoc(doc.title);
            setSelectedDoc(doc);
        }
        setAction(action);
        setOpen(true);
    };

    const darkMode = localStorage.getItem("darkMode") === "true";

    return (
        <>
            <Dialog placeholder={"dialog"} open={open} handler={() => setOpen(!open)}>
                <DialogHeader placeholder="dialog-header">
                    <h5 className="text-lg font-bold">{action === "create" ? "Create Document" : "Update Document"}</h5>
                </DialogHeader>
                <DialogBody placeholder="dialog-body">
                    <Input crossOrigin="true" placeholder="Title" label="Document Title" onChange={(e) => setNewDoc(e.target.value)} />
                </DialogBody>
                <DialogFooter placeholder="dialog-footer">
                    <Button
                        placeholder="button"
                        onClick={() => handleCreateOrUpdate(action, newDoc, selectedDoc)} // pass the first document as a placeholder
                        className="text-white bg-blue-500 hover:bg-blue-600 active:bg-blue-700 px-4 py-2 rounded-md"
                    >
                        {action === "create" ? "Create" : "Update"}
                    </Button>
                </DialogFooter>
            </Dialog>
            {showAlert && (
                <div className="fixed bottom-0 left-0 right-0 flex justify-center items-center p-4 z-50 transform transition-transform duration-300 ease-in-out">
                    <Alert
                        animate={{
                            mount: { y: 0 },
                            unmount: { y: 100 }
                        }}
                        onClose={() => setShowAlert(false)}
                        className="w-96"
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
                <Card placeholder="card" color={darkMode ? "gray" : "white"} shadow={true} className="h-full w-full p-4">
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
                        <IconButton onClick={(e) => handleOpenDialog(e, "create", documents[0])} variant="text" color="blue" placeholder="create">
                            <DocumentPlusIcon className="h-5 w-5" />
                        </IconButton>
                    </List>
                </Card>
            </Drawer>
        </>
    );
}
