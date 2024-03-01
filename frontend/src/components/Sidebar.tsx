import React from "react";
import { IconButton, List, ListItem, ListItemPrefix, Input, Drawer, Card } from "@material-tailwind/react";
import { DocumentIcon } from "@heroicons/react/24/solid";
import { MagnifyingGlassIcon, Bars3Icon, XMarkIcon } from "@heroicons/react/24/outline";
import { getDocuments, Document } from "../utils";

export default function Sidebar() {
    const [isDrawerOpen, setIsDrawerOpen] = React.useState(false);
    const [documents, setDocuments] = React.useState<Document[]>([]);

    React.useEffect(() => {
        getDocuments().then((data) => setDocuments(data));
    }, []);

    const openDrawer = () => setIsDrawerOpen(true);
    const closeDrawer = () => setIsDrawerOpen(false);
    const getMenuItems = () => {
        return documents.map((document) => (
            <ListItem key={document.uuid} placeholder="list-item">
                <ListItemPrefix placeholder="list-item-prefix">
                    <DocumentIcon className="h-5 w-5" />
                </ListItemPrefix>
                <p>{document.title}</p>
            </ListItem>
        ));
    };

    return (
        <>
            <IconButton placeholder="icon-button" variant="text" size="lg" onClick={openDrawer}>
                {isDrawerOpen ? <XMarkIcon className="h-8 w-8 stroke-2" /> : <Bars3Icon className="h-8 w-8 stroke-2" />}
            </IconButton>
            <Drawer placeholder="drawer" open={isDrawerOpen} onClose={closeDrawer} overlay={false}>
                <Card placeholder="card" color="transparent" shadow={true} className="h-[calc(100vh-2rem)] w-full p-4">
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
