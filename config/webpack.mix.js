const mix = require("laravel-mix");
const path = require("path");
const tailwindCss = require("tailwindcss");

mix.options({manifest: false})
    .ts(["src/app.ts", "src/dump.ts"], "dist/quo-runtime.js", {
        configFile: path.resolve(__dirname, "tsconfig.json"),
    })
    .sass("src/Resources/style/quo.scss", "dist/quo.css")
    .options({
        postCss: [tailwindCss("config/tailwind.config.js")],
    })
    .copyDirectory("src/Resources/fonts/", "dist/fonts/")
    .minify("dist/quo-runtime.js")
    .minify("dist/quo.css");
