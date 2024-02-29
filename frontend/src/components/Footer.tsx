import { Typography } from "@material-tailwind/react";

const getYear = () => {
    return new Date().getFullYear();
};

export default function Footer() {
    return (
        <footer className="flex w-full flex-col flex-wrap items-center justify-center gap-y-6 gap-x-12 py-6 text-center md:justify-between">
            <ul className="flex flex-col flex-wrap items-center gap-y-2 gap-x-8">
                <li>
                    <Typography
                        placeholder="Markdown Edit"
                        color="blue-gray"
                        className="font-normal transition-colors hover:text-blue-500 focus:text-blue-500"
                    >
                        &copy; {getYear()} Markdown Edit
                    </Typography>
                </li>
                <li>
                    <Typography
                        placeholder="Made with ❤️ by Maneesh"
                        color="blue-gray"
                        className="font-normal transition-colors hover:text-blue-500 focus:text-blue-500"
                    >
                        Made with ❤️ by{" "}
                        <a
                            href="https://github.com/ManeeshWije"
                            target="_blank"
                            rel="noreferrer"
                            className="transition-colors hover:text-blue-500 focus:text-blue-500"
                        >
                            Maneesh
                        </a>
                    </Typography>
                </li>
            </ul>
        </footer>
    );
}
