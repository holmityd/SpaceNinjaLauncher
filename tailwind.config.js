/** @type {import('tailwindcss').Config} */
const config = {
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
    ],

    plugins: [require("flowbite/plugin")],

    darkMode: "class",

    theme: {
        extend: {
            colors: {
                // flowbite-svelte
                primary: {
                    50: "#FFF5F2",
                    100: "#FFF1EE",
                    200: "#1d4ed8",
                    300: "#FFD5CC",
                    400: "#FFBCAD",
                    500: "#1d4ed8",
                    600: "#111827",
                    700: "#1f2937",
                    800: "#1f2937",
                    900: "#A5371B",
                },
            },
        },
    },
};

module.exports = config;
