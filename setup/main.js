const {app, BrowserWindow, globalShortcut, Menu, shell, ipcMain} = require("electron");
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
    mainWindow.webContents.on("will-navigate", e => e.preventDefault());

    mainWindow.loadFile(path.join(__dirname, "../src/Resources/main.html"))
        .then(() => {
            if (process.env.NODE_ENV === "development") {
                try {
                    require("electron-reloader")(module);
                } catch {
                    //
                }
                mainWindow.openDevTools();
            } else {
                globalShortcut.unregisterAll();
                Menu.setApplicationMenu(null);
                mainWindow.closeDevTools();
            }
        })
        .catch(err => {
            if (process.env.NODE_ENV === "development") {
                console.error(err);
            }
        });
}

app.whenReady().then(() => {
    createWindow();

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

ipcMain.on("quo-open-link", (e, args) => {
    if (args) {
        if (args.includes("phpstorm:")) {
            shell.openExternal(args);
        }
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
            response.end("{\"quo-server-says\": \"Hi!\"}");

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
