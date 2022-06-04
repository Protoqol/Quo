module.exports = {
    "packagerConfig": {
        "name": "QuoClient",
        "out": "./build"
    },
    "makers": [
        {
            "name": "@electron-forge/maker-squirrel",
            "config": {
                "name": "QuoClient",
            }
        }
    ]
};