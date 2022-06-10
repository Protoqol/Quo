import QuoState from "../State/QuoState";

declare global {
    interface Window {
        MainProcess: {
            incomingPayload: {
                (callback: Function): void
            };

            openUrl: any;
        };

        QuoState: QuoState;

        Sfdump: any;
    }
}

export {};
