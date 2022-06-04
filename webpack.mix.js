let mix = require("laravel-mix");
require("mix-tailwindcss");

mix.copy("src/app.js", "dist/app.js")
    .copy("src/dumper/dump.js", "dist/dumper/dump.js")
    .sass("src/dumper/dump.scss", "dist/dumper/dump.css")
    .sass("src/quo.scss", "dist/quo.css")
    .tailwind("tailwind.config.js");
