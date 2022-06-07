const path = require("path");

module.exports = {
    "packagerConfig": {},
    "makers"        : [
        {
            name  : "@electron-forge/maker-deb",
            config: {
                options: {
                    "owner"             : "Protoqol",
                    "maintainer"        : "Protoqol",
                    "name"              : "Quo",
                    "genericName"       : "Debugger",
                    "productName"       : "Quo",
                    "categories"        : ["Development"],
                    "productDescription": "Quo is a debugging utility to easily dump variables, the dumped variables will appear in this Quo client instead of the traditional way which is often tedious.",
                    "description"       : "Quo, debugging software with ease.",
                    "icon"              : path.resolve(__dirname + "/build/ico/ico-quo.png"),
                    "section"           : "devel",

                },
            },
        },
        {
            name  : "@electron-forge/maker-dmg",
            config: {
                "name"                : "Quo",
                "overwrite"           : true,
                "additionalDMGOptions": {
                    "title"   : "Quo installation",
                    "icon"    : path.resolve(__dirname + "/build/ico/ico-quo.icns"),
                    "contents": [
                        {"x": 448, "y": 344, "type": "link", "path": "/Applications"},
                        {"x": 192, "y": 344, "type": "file", "path": "Quo Client.app"},
                    ],
                },
            },
        },
        {
            name  : "@electron-forge/maker-squirrel",
            config: {
                "name"       : "quo",
                "description": "Quo, debugging software with ease.",
                "exe"        : "Quo.exe",
                "iconUrl"    : path.resolve(__dirname + "/build/ico/ico-quo.ico"),
                "setupExe"   : "quo-installer.exe",
                "title"      : "Quo",
                "productName": "Quo",

            },
        },
        {
            name: "@electron-forge/maker-zip",
        },
    ],
    "publishers"    : [
        {
            name      : "@electron-forge/publisher-github",
            config    : {
                "repository": {
                    "owner": "protoqol",
                    "name" : "quo",
                },
            },
            draft     : true,
            prerelease: true,
        },
    ],
};
