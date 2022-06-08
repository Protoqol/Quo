const colors = require('tailwindcss/colors')

module.exports = {
    content: [
        'src/Resources/main.html'
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
