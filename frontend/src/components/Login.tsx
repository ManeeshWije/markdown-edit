import GoogleButton from "react-google-button";
export default function Login() {
    const authUrl = import.meta.env.VITE_SERVER_URL + "/auth/google/login";
    return (
        <div className="flex justify-center items-center">
            <a href={authUrl}>
                <GoogleButton />
            </a>
        </div>
    );
}
