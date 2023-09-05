import { useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { githubDark, githubLight } from "@uiw/codemirror-theme-github";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import Tools from "./components/Tools";
import "./input.css";

export default function App() {
    const [markdownContent, setMarkdownContent] = useState("");
    const [showPreview, setShowPreview] = useState(false);
    const [darkMode, setDarkMode] = useState(false);

    const handleCodeMirrorChange = (value: string) => {
        setMarkdownContent(value);
        localStorage.setItem("markdownContent", value);
    };

    const togglePreview = () => {
        setShowPreview(!showPreview);
    };

    const toggleDarkMode = () => {
        setDarkMode(!darkMode);
        document.body.classList.toggle("dark-mode");
    };

    return (
        <div>
            <Tools
                onTogglePreview={togglePreview}
                onToggleDarkMode={toggleDarkMode}
                darkMode={darkMode}
            />
            <div
                className={`editor-container ${
                    showPreview ? "preview-visible" : ""
                }`}
            >
                <CodeMirror
                    value={
                        localStorage.getItem("markdownContent") ||
                        markdownContent
                    }
                    autoFocus
                    onChange={handleCodeMirrorChange}
                    extensions={[
                        markdown({
                            base: markdownLanguage,
                            codeLanguages: languages,
                        }),
                    ]}
                    className="flex-1"
                    theme={darkMode ? githubDark : githubLight}
                />
                {showPreview && (
                    <div className="preview markdown">
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>
                            {localStorage.getItem("markdownContent") ||
                                markdownContent}
                        </ReactMarkdown>
                    </div>
                )}
            </div>
        </div>
    );
}
