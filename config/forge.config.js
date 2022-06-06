module.exports = {
    "packagerConfig": {
        "name": "QuoClient",
        "out" : "./build",
    },
    "makers"        : [
        {
            name  : "@electron-forge/maker-deb",
            config: {
                options: {
                    "owner"             : "Protoqol",
                    "maintainer"        : "Protoqol",
                    "name"              : "quo",
                    "genericName"       : "Debugger",
                    "productName"       : "Quo client",
                    "categories"        : "Development",
                    "productDescription": "Quo is a debugging utility to easily dump variables, the dumped variables will appear in this Quo client instead of the traditional way which is often tedious.",
                    "description"       : "Quo client, debugging software with ease.",
                    "icon"              : "./config/build/ico-quo.svg",
                    "section"           : "devel",

                },
            },
        },
        {
            name  : "@electron-forge/maker-dmg",
            config: {
                "name"                : "Quo Client",
                "overwrite"           : true,
                "additionalDMGOptions": {
                    "title"   : "Quo installation",
                    "icon"    : "./config/build/ico-quo.icns",
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
                "description": "Quo client, debugging software with ease.",
                "exe"        : "quo-client.exe",
                "icon"       : "./config/build/ico-quo.ico",
                "setupExe"   : "quo-client-installer.exe",
                "title"      : "Quo client",
                "productName": "Quo client",

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
