import {QuoDom} from "./Classes/QuoDom";
import "./Interfaces/Window";
import QuoUI from "./Classes/QuoUI";

var xmlHttp = new XMLHttpRequest();
xmlHttp.open("GET", "http://dashboard.test", true); // false for synchronous request
xmlHttp.send(null);

window.UI = new QuoUI();
window.quoTunnel.incomingPayload(QuoDom.make);
window.addEventListener("DOMContentLoaded", QuoDom.registerHandlers);
