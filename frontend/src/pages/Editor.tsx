import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Tools from "../components/Tools";
import { Document, getDocuments } from "../utils";
import "../input.css";

export default function Editor() {
    const [markdownContent, setMarkdownContent] = React.useState("");
    const [showPreview, setShowPreview] = React.useState(false);
    const [darkMode, setDarkMode] = React.useState(false);

    React.useEffect(() => {
        const fetchDocumentContent = async () => {
            await getDocumentContent();
        };
        const isDarkMode = localStorage.getItem("darkMode") === "true";
        setDarkMode(isDarkMode);
        document.body.classList.toggle("dark-mode", isDarkMode);
        fetchDocumentContent();
    }, []);

    const handleCodeMirrorChange = (value: string) => {
        setMarkdownContent(value);
    };

    const getDocumentContent = async () => {
        const documents: Document[] = await getDocuments();
        if (documents.length > 0) {
            setMarkdownContent(documents[0].content);
        }
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

    const updateEditorContent = (content: string) => {
        setMarkdownContent(content);
    };

    return (
        <div className={`editor-container ${showPreview ? "preview-visible" : ""}`}>
            <Tools onTogglePreview={togglePreview} onToggleDarkMode={toggleDarkMode} darkMode={darkMode} onDocumentClick={updateEditorContent} />
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
        </div>
    );
}
