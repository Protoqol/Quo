import {QuoDom} from "./Classes/QuoDom";
import "./Interfaces/Window";

window.quoTunnel.incomingPayload(QuoDom.make);
window.addEventListener("DOMContentLoaded", QuoDom.registerHandlers);

