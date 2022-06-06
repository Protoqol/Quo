let mix = require("laravel-mix");
require("mix-tailwindcss");

mix.combine(["src/app.js", "src/dumper/dump.js"], "dist/quo-runtime.js")
    .sass("src/quo.scss", "dist/quo.css")
    .tailwind("config/tailwind.config.js")
    .copyDirectory("src/fonts/", "dist/fonts/")
    .minify("dist/quo-runtime.js")
    .minify("dist/quo.css");
