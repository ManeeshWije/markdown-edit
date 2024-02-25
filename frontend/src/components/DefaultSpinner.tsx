import { Spinner } from "@material-tailwind/react";

export function DefaultSpinner() {
    return (
        <div className="flex justify-center items-center h-screen">
            <Spinner color="blue" />
        </div>
    );
}
