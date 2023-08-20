import { Navbar, Typography, Button } from "@material-tailwind/react";

interface ToolsProps {
    onTogglePreview: () => void;
}

const Tools: React.FC<ToolsProps> = ({ onTogglePreview }) => {
    return (
        <Navbar className="sticky top-0 z-10 h-max max-w-full rounded-none py-2 px-4 lg:px-8 lg:py-4">
            <div className="flex items-center justify-between text-blue-gray-900">
                <Typography
                    as="a"
                    href="#"
                    className="mr-4 cursor-pointer py-1.5 font-medium"
                >
                    Markdown Edit: By Maneesh Wije
                </Typography>
                <Button
                    variant="gradient"
                    size="sm"
                    className="lg:inline-block"
                    onClick={onTogglePreview}
                >
                    Toggle Preview
                </Button>
            </div>
        </Navbar>
    );
};

export default Tools;