import React from "react";
import CodeMirror from "@uiw/react-codemirror";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Tools from "../components/Tools";
import "../input.css";

export default function Editor() {
    const [markdownContent, setMarkdownContent] = React.useState("");
    const [showPreview, setShowPreview] = React.useState(false);
    const [darkMode, setDarkMode] = React.useState(false);

    React.useEffect(() => {
        const isDarkMode = localStorage.getItem("darkMode") === "true";
        setDarkMode(isDarkMode);
        document.body.classList.toggle("dark-mode", isDarkMode);
    }, []);

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

    return (
        <div>
            <Tools onTogglePreview={togglePreview} onToggleDarkMode={toggleDarkMode} darkMode={darkMode} />
            <div className={`editor-container ${showPreview ? "preview-visible" : ""}`}>
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
                    className="flex-1"
                    theme={darkMode ? githubDark : githubLight}
                />
                {showPreview && (
                    <div className="preview markdown">
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>{markdownContent}</ReactMarkdown>
                    </div>
                )}
            </div>
        </div>
    );
}
