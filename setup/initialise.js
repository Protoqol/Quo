const {contextBridge, ipcRenderer} = require("electron");

contextBridge.exposeInMainWorld("quoTunnel", {
    on: (callback) => {
        ipcRenderer.on("quo-tunnel", callback);
    },
});
