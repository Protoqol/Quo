export interface IncomingPayloadInterface {
    meta: {
        id: number,
        uid: string,
        calledVariable: string,
        origin: string,
        senderOrigin: string,
        time: string,
    };

    data: string;
}
