// eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
const withMT = require("@material-tailwind/react/utils/withMT");
export default withMT({
    content: [
        "./src/*.tsx",
        "./src/**/*.tsx",
        "./src/**/**/*.tsx",
        "index.html",
        "./src/**/*.html",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
    darkMode: "class",
});
