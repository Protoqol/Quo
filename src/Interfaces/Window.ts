declare global {
    interface Window {
        quoTunnel: {
            incomingPayload: {
                (callback: Function): void
            }
        };

        Sfdump: any;
    }
}

export {};
