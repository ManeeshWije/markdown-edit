import { Navbar, Button } from "@material-tailwind/react";
import Sidebar from "./Sidebar";
import Logout from "./Logout";
import { exportToHTML } from "../utils";
import { useStore } from "../store";
import React from "react";

interface ToolsProps {
    onTogglePreview: () => void;
    onToggleDarkMode: () => void;
    darkMode: boolean;
}

const Tools: React.FC<ToolsProps> = ({ onTogglePreview, onToggleDarkMode, darkMode }) => {
    const { selectedDoc } = useStore();

    const selectedDocRef = React.useRef(selectedDoc);

    React.useEffect(() => {
        selectedDocRef.current = selectedDoc;
    }, [selectedDoc]);

    return (
        <>
            <Navbar
                placeholder="navbar"
                className={`sticky top-0 z-10 h-max max-w-full rounded-none py-2 px-4 lg:px-8 lg:py-4 ${darkMode ? "bg-gray-900 text-white" : "bg-white text-blue-gray-900"}`}
            >
                <div className="flex flex-wrap items-center justify-evenly">
                    <Sidebar />
                    <Button placeholder="Toggle Preview" variant="gradient" size="sm" className="lg:inline-block" onClick={onTogglePreview}>
                        Toggle Preview
                    </Button>
                    <Button placeholder="Toggle Dark Mode" variant="gradient" size="sm" className="lg:inline-block" onClick={onToggleDarkMode}>
                        Toggle Dark Mode
                    </Button>
                    <Button
                        placeholder="Export"
                        variant="gradient"
                        size="sm"
                        className="lg:inline-block"
                        onClick={() => {
                            if (selectedDocRef.current.uuid === selectedDoc.uuid) {
                                exportToHTML(selectedDoc.title, selectedDoc.content);
                            }
                        }}
                    >
                        Export to HTML
                    </Button>
                    <Logout />
                </div>
            </Navbar>
        </>
    );
};

export default Tools;
