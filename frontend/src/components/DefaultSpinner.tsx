import { Spinner } from "@material-tailwind/react";

export function DefaultSpinner() {
    return (
        <div className="flex justify-center items-center h-screen">
            <Spinner color="blue" width={100} height={100} />
        </div>
    );
}
