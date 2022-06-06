const colors = require('tailwindcss/colors')

module.exports = {
    content: [
        'index.html',
    ],
    theme  : {
        extend: {
            colors: {
                gray   : colors.slate,
                emerald: colors.emerald,
                violet : colors.violet,
            },
        },
    },
    plugins: [],
}
