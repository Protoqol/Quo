import {IncomingPayloadInterface} from "../Abstract/IncomingPayloadInterface";
import "../Abstract/Window";
import DOM from "../DOM";
import {HTMLDecorator} from "../Decorators/HTMLDecorator";
import PayloadData from "./Data/PayloadData";

export default class Payload extends HTMLDecorator {
    /**
     * @type {Element}
     * @private
     */
    public element: HTMLElement;

    /**
     *
     * @type {PayloadData}
     * @private
     */
    public data: PayloadData;

    /**
     * @param {IncomingPayloadInterface} incomingPayload
     */
    constructor(incomingPayload: IncomingPayloadInterface) {
        super();
        this.data = new PayloadData(incomingPayload);
    }

    /**
     * Admit an incoming payload to the list.
     *
     * @param e
     * @param {IncomingPayloadInterface} incomingPayload
     */
    public static admit(e: Event, incomingPayload: IncomingPayloadInterface) {
        let payload = new Payload(incomingPayload);
        window.QuoState.savePayloadToState(payload);

        DOM.hideEmptyListMessage();
        DOM.addHtmlToList(payload);
        DOM.afterAdditionHandler(payload);
    }
}
