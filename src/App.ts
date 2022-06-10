import QuoState from "./State/QuoState";
import DOM from "./DOM";
import Payload from "./Entities/Payload";
import "./Abstract/Window";

const xmlHttp = new XMLHttpRequest();
xmlHttp.open("GET", "http://localhost:8001", true); // false for synchronous request
xmlHttp.send(null);
xmlHttp.open("GET", "http://dashboard.test", true); // false for synchronous request
xmlHttp.send(null);

window.QuoState = new QuoState();
window.MainProcess.incomingPayload(Payload.admit);
window.addEventListener("DOMContentLoaded", DOM.registerHandlers);
