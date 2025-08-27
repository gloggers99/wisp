/** @type {import('tailwindcss').Config} */

module.exports = {
    content: [
        "./**/*.rs"
    ],
    theme: {
        extend: {
            fontFamily: {
                lato: ['Lato', 'sans-serif']
            },
            colors: {
                oxocarbon: {
                    background:  '#161616',
                    1:  '#262626',
                    2:  '#393939',
                    3:  '#525252',
                    4:  '#dde1e6',
                    foreground:  '#f2f4f8',
                    6:  '#ffffff',
                    teal:  '#08bdba',
                    8:  '#3ddbd9',
                    blue:  '#78a9ff',
                    magenta:  '#ee5396',
                    radiant_blue:  '#33b1ff',
                    pink:  '#ff7eb6',
                    green:  '#42be65',
                    purple:  '#be95ff',
                    pastel_blue:  '#82cfff',
                },
            },
        },
    },
    plugins: [],
}