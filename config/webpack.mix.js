let mix = require("laravel-mix");
let tailwindCss = require("tailwindcss");

mix.options({manifest: false})
    .combine(["src/app.js", "src/dumper/dump.js"], "dist/quo-runtime.js")
    .sass("src/quo.scss", "dist/quo.css")
    .options({
        postCss: [tailwindCss("config/tailwind.config.js")],
    })
    .copyDirectory("src/fonts/", "dist/fonts/")
    .minify("dist/quo-runtime.js")
    .minify("dist/quo.css");
