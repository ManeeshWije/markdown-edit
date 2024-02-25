import GoogleButton from "react-google-button";
export default function Login() {
    const authUrl = import.meta.env.VITE_SERVER_URL + "/auth/google/login";
    return (
        // center the button
        <div className="flex justify-center items-center h-screen">
            <a href={authUrl}>
                <GoogleButton />
            </a>
        </div>
    );
}
