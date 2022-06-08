import {QuoTunnelInterface} from "./QuoTunnelInterface";
import QuoUI from "../Classes/QuoUI";

declare global {
    interface Window {
        quoTunnel: QuoTunnelInterface;

        UI: QuoUI;

        Sfdump: any;
    }
}

export {};
