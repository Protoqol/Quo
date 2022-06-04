const {app, BrowserWindow, globalShortcut, Menu} = require("electron");
const {Buffer} = require("node:buffer");
const path = require("path");
const http = require("http");

let mainWindow = null;

function createWindow() {
    mainWindow = new BrowserWindow({
        width         : 1600,
        height        : 900,
        minWidth      : 935,
        minHeight     : 555,
        icon          : path.join(__dirname, "meta/quo.png"),
        webPreferences: {
            preload: path.join(__dirname, "initialise.js"),
        },
    });

    mainWindow.loadFile("main.html");

    if (process.env.ELECTRON_ENV === "development") {
        mainWindow.webContents.openDevTools();
    }
}

app.whenReady().then(() => {
    createWindow();

    globalShortcut.unregisterAll();
    Menu.setApplicationMenu(null);

    app.on("activate", function () {
        if (BrowserWindow.getAllWindows().length === 0) {
            createWindow();
        }
    });

    app.on("new-window-for-tab", () => false);
    app.on("will-navigate", () => false);
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") {
        app.quit();
    }
});

if (process.env.ELECTRON_ENV === "development") {
    try {
        require("electron-reloader")(module);
    } catch (_) {
        //
    }
}

// Server
http.createServer((request, response) => {
    let requestData = "";
    request.on("readable", () => {
        requestData += request.read();
    });

    request.on("end", () => {
        let message = JSON.parse(requestData.replace("null", ""));
        let buff = Buffer.from(message.dump, "base64");

        mainWindow.webContents.send("quo-tunnel", {
            data  : buff.toString(),
            detail: message,
        });

        response.setHeader("Content-Type", "application/json");
        response.writeHead(200);
        response.end("{\"message\": \"ok\"}");
    });
}).listen(8118, "127.0.0.1");
