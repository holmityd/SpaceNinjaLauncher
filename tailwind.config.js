/** @type {import('tailwindcss').Config} */
export default {
    content: ["./src/**/*.{html,js,svelte,ts}"],
    theme: {
        extend: {
            dropShadow: {
                glow: ["0 0px 1px rgba(255,255, 255, 0.35)", "0 0px 3px rgba(255, 255,255, 0.2)"],
            },
        },
    },
    plugins: [],
};
