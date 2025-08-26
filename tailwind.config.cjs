/** @type {import('tailwindcss').Config} */

console.log("ASDFASDF")
module.exports = {
    content: [
        "./src/**/*.{rs,html}",
        "./beigebox_core/**/*.{rs,html}",
        "./beigebox_database/**/*.{rs,html}",
        "./beigebox_session_manager/**/*.{rs,html}",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
}