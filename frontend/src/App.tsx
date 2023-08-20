import { useState } from "react";
import CodeMirror from "@uiw/react-codemirror";
import { markdown, markdownLanguage } from "@codemirror/lang-markdown";
import { languages } from "@codemirror/language-data";
import ReactMarkdown from "react-markdown";
import remarkGfm from 'remark-gfm'
import Tools from "./components/Tools";
import "./input.css";

export default function App() {
    const [markdownContent, setMarkdownContent] = useState("");
    const [showPreview, setShowPreview] = useState(false);

    const handleCodeMirrorChange = (value: string) => {
        setMarkdownContent(value);
    };

    const togglePreview = () => {
        setShowPreview(!showPreview);
    };

    return (
        <div>
            <Tools onTogglePreview={togglePreview} />
            <div
                className={`editor-container ${
                    showPreview ? "preview-visible" : ""
                }`}
            >
                <CodeMirror
                    value={markdownContent}
                    autoFocus
                    onChange={handleCodeMirrorChange}
                    extensions={[
                        markdown({
                            base: markdownLanguage,
                            codeLanguages: languages,
                        }),
                    ]}
                    className="flex-1"
                />
                {showPreview && (
                    <div className="preview markdown">
                        <ReactMarkdown remarkPlugins={[remarkGfm]}>
                            {markdownContent}
                        </ReactMarkdown>
                    </div>
                )}
            </div>
        </div>
    );
}
