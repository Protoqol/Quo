const {contextBridge, ipcRenderer} = require("electron");

contextBridge.exposeInMainWorld("quoTunnel", {
    incomingPayload: (callback) => {
        ipcRenderer.on("quo-tunnel", callback);
    },
});
