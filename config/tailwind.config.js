const colors = require('tailwindcss/colors')

module.exports = {
    content: [
        'setup/main.html'
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
