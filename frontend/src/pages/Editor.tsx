import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { basicSetup } from "@uiw/react-codemirror";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Tools from "../components/Tools";
import { updateDocument } from "../utils";
import "../input.css";
import { Alert } from "@material-tailwind/react";
import { useStore } from "../store";

export default function Editor() {
    const { documents, selectedDoc, setSelectedDoc } = useStore();
    const [markdownContent, setMarkdownContent] = React.useState("");
    const [showPreview, setShowPreview] = React.useState(false);
    const [darkMode, setDarkMode] = React.useState(false);
    const [hasDocument, setHasDocument] = React.useState(false);

    // if there are documents, set the first document as the selected document
    React.useEffect(() => {
        if (documents.length > 0) {
            setHasDocument(true);
            if (Object.keys(selectedDoc).length === 0 && selectedDoc.constructor === Object) {
                setSelectedDoc(documents[0]);
                setMarkdownContent(documents[0].content);
            }
        } else {
            setHasDocument(false);
        }
        const isDarkMode = localStorage.getItem("darkMode") === "true";
        setDarkMode(isDarkMode);
        document.body.classList.toggle("dark-mode", isDarkMode);
    }, [documents, selectedDoc, setSelectedDoc]);

    // update the document content when the markdown content changes
    React.useEffect(() => {
        const timer = setTimeout(() => {
            if (hasDocument) {
                updateDocument(selectedDoc.uuid, selectedDoc.title, markdownContent);
                documents.forEach((doc) => {
                    if (doc.uuid === selectedDoc.uuid) {
                        doc.content = markdownContent;
                    }
                });
            }
        }, 500);
        return () => clearTimeout(timer);
    }, [markdownContent, selectedDoc, hasDocument, documents]);

    const handleCodeMirrorChange = (value: string) => {
        setMarkdownContent(value);
    };

    const togglePreview = () => {
        setShowPreview(!showPreview);
    };

    const toggleDarkMode = () => {
        const newDarkMode = !darkMode;
        setDarkMode(newDarkMode);
        document.body.classList.toggle("dark-mode", newDarkMode);
        localStorage.setItem("darkMode", JSON.stringify(newDarkMode));
    };

    const emptyDocuments = () => {
        return (
            <div className="alert">
                <Alert color="gray">You must create a document first!</Alert>
            </div>
        );
    };

    return (
        <div className={`editor-container ${showPreview ? "preview-visible" : ""}`}>
            <Tools onTogglePreview={togglePreview} onToggleDarkMode={toggleDarkMode} darkMode={darkMode} onDocumentClick={handleCodeMirrorChange} />
            {hasDocument ? (
                <div className="editor-wrapper">
                    <div className="editor-column">
                        <CodeMirror
                            value={markdownContent}
                            autoFocus
                            onChange={handleCodeMirrorChange}
                            extensions={[
                                markdown({
                                    base: markdownLanguage,
                                    codeLanguages: languages
                                }),
                                basicSetup({
                                    foldGutter: false,
                                    dropCursor: false,
                                    allowMultipleSelections: false,
                                    indentOnInput: false
                                })
                            ]}
                            theme={darkMode ? githubDark : githubLight}
                        />
                    </div>
                    <div className={`preview-column ${showPreview ? "preview-visible" : ""}`}>
                        {showPreview && (
                            <div className="preview markdown">
                                <ReactMarkdown remarkPlugins={[remarkGfm]}>{markdownContent}</ReactMarkdown>
                            </div>
                        )}
                    </div>
                </div>
            ) : (
                emptyDocuments()
            )}
        </div>
    );
}
