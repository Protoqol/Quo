/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.{rs,js}", "./index.html", "./styles.scss", "./dump.scss"],
    theme: {
        fontFamily: {
            mono: ['Google-Sans', 'ui-monospace', 'SFMono-Regular', 'Menlo', 'Monaco', 'Consolas', 'monospace']
        },
        extend: {
            colors: {
                accent: "#ec135b"
            }
        },
    },
    plugins: [],
}

