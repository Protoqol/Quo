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
        icon          : path.join(__dirname, "../config/build/ico/ico-quo.ico"),
        webPreferences: {
            sandbox         : true,
            preload         : path.join(__dirname, "initialise.js"),
            contextIsolation: true,
        },
    });

    mainWindow.webContents.setWindowOpenHandler(() => ({action: "deny"}));
    mainWindow.webContents.on("new-window", e => e.preventDefault());
    mainWindow.webContents.on("will-navigate", e => e.preventDefault());
    mainWindow.loadFile(path.join(__dirname, "main.html"));
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
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") {
        app.quit();
    }
});

http.createServer((request, response) => {
    let requestData = "";

    request.on("readable", () => {
        requestData += request.read();
    });

    request.on("end", () => {
        if (request.url !== "/quo-tunnel") {
            response.setHeader("Content-Type", "application/json");
            response.writeHead(200);
            response.end("{\"quo-server\": \"Hi!\"}");

            return true;
        }

        let data = JSON.parse(requestData.replace("null", ""));
        let buff = Buffer.from(data.payload, "base64");

        mainWindow.webContents.send("quo-tunnel", {
            data: buff.toString(),
            meta: data.meta,
        });

        response.setHeader("Content-Type", "application/json");
        response.writeHead(200);
        response.end("{\"message\": \"ok\"}");
    });
}).listen(7312, "127.0.0.1");
