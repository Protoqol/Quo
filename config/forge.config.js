module.exports = {
    "packagerConfig": {
        "name": "QuoClient",
        "out" : "./build",
    },
    "makers"        : [
        {
            "name"  : "@electron-forge/maker-squirrel",
            "config": {
                "name": "QuoClient",
            },
        },
    ],
    "publishers"    : [
        {
            "name"  : "@electron-forge/publisher-github",
            "config": {
                "repository": {
                    "owner": "protoqol",
                    "name" : "quo",
                },
            },
            "draft" : true,
        },
    ],
};
