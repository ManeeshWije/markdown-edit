import { Spinner } from "@material-tailwind/react";

export function DefaultSpinner() {
    const isDarkMode = localStorage.getItem("darkMode") === "true";
    document.body.classList.toggle("dark-mode", isDarkMode);

    return (
        <div id="home" className="flex justify-center items-center h-screen">
            <Spinner color="blue" width={100} height={100} />
        </div>
    );
}
