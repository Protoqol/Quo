import QuoState from "./State/QuoState";
import DOM from "./DOM";
import Payload from "./Entities/Payload";
import "./Abstract/Window";

window.QuoState = new QuoState();
window.MainProcess.incomingPayload(Payload.admit);
window.addEventListener("DOMContentLoaded", DOM.registerHandlers);
