const {contextBridge, ipcRenderer} = require("electron");

contextBridge.exposeInMainWorld("quoTunnel", {
    incomingPayload: (callback) => {
        ipcRenderer.on("quo-tunnel", callback);
    },

    openUrl: (url) => {
        ipcRenderer.send("quo-open-link", url);
    },
});
