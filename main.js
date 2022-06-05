/* eng-disable LIMIT_NAVIGATION_GLOBAL_CHECK */
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
        icon          : path.join(__dirname, "meta/ico/quo.png"),
        webPreferences: {
            sandbox         : true,
            preload         : path.join(__dirname, "initialise.js"), /* eng-disable PRELOAD_JS_CHECK */
            contextIsolation: true,
        },
    });

    mainWindow.webContents.setWindowOpenHandler(() => ({action: "deny"}));
    mainWindow.webContents.on("new-window", e => e.preventDefault()); /* eng-disable LIMIT_NAVIGATION_JS_CHECK */
    mainWindow.webContents.on("will-navigate", e => e.preventDefault()); /* eng-disable LIMIT_NAVIGATION_JS_CHECK */

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

    mainWindow.openDevTools();
});

app.on("window-all-closed", function () {
    if (process.platform !== "darwin") {
        app.quit();
    }
});

// Server
http.createServer((request, response) => {
    let requestData = "";

    request.on("readable", () => {
        requestData += request.read();
    });

    request.on("end", () => {
        if (request.url !== '/quo-tunnel') {
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
