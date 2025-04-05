import GoogleButton from "react-google-button";
export default function Login() {
    const serverUrl = import.meta.env.MODE === "production" ? "" : "http://localhost:8080/auth/google/login"
    return (
        <div className="flex justify-center items-center">
            <a href={serverUrl}>
                <GoogleButton />
            </a>
        </div>
    );
}
