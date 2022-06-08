import {QuoDom} from "./Classes/QuoDom";
import "./Interfaces/Window";
import QuoUI from "./Classes/QuoUI";

window.UI = new QuoUI();
window.quoTunnel.incomingPayload(QuoDom.make);
window.addEventListener("DOMContentLoaded", QuoDom.registerHandlers);
