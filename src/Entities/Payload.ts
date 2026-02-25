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
     * @type {PayloadData}
     * @private
     */
    public data: PayloadData;

    /**
     * @type {boolean}
     */
    public isCloaked: boolean;

    /**
     * @type {boolean}
     */
    public isUncloaked: boolean;

    /**
     * @param {IncomingPayloadInterface} incomingPayload
     */
    constructor(incomingPayload: IncomingPayloadInterface) {
        super();
        this.data = new PayloadData(incomingPayload);
        this.element = this.makeElement();
    }

    /**
     * Admit an incoming payload to the list.
     *
     * @param e
     * @param {IncomingPayloadInterface} incomingPayload
     */
    public static admit(e: Event, incomingPayload: IncomingPayloadInterface) {
        let payload = new Payload(incomingPayload);
        window.QuoState.admitPayloadToState(payload);

        DOM.hideEmptyListMessage();
        DOM.addHtmlToList(payload);
    }

    /**
     * Create payload element.
     *
     * @returns {HTMLElement}
     */
    public makeElement(): HTMLElement {
        const dumpContainer = document.createElement("div");
        dumpContainer.id = `${this.data.getUuid()}`;
        dumpContainer.dataset.domain = `${this.data.getSenderOrigin()}`;
        dumpContainer.dataset.request = `${this.data.getUid()}`;
        dumpContainer.classList.add("quo-dump-container");
        dumpContainer.classList.add("flex");
        dumpContainer.addEventListener("click", (e: MouseEvent) => {
            window.MainProcess.openUrl((e.target as HTMLAnchorElement).href);
        });
        dumpContainer.innerHTML = `
            <div class="time">
                <span>${this.data.getSenderOrigin()} • ${this.data.getTime()}</span>
                <a id="quo-link-${this.data.getUid()}" target="_top" href="phpstorm://open?file=${this.data.getOriginWithoutLineNr()}&line=${this.data.getLineNr()}">${this.data.getOrigin()}</a>
            </div>
            <div class="quo-actual-dump">
                <div class="dumps">
                    <div class="grouped-dump-wrapper">
                        <div class="quo-title">
                            <div class="file">
                                <div class="backtrace">
                                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="14" height="14" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 18l-6-6 6-6"/></svg>
                                    <span>Arg #${this.data.getNoOfNodes() + 1} of ${this.data.getAllPassedVariables().length}</span>
                                </div>
                                <div class="passed" style="${this.data.getVarname() ? "" : "display:none;"}">
                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18" fill="currentColor"><path d="M13.172 12l-4.95-4.95 1.414-1.414L16 12l-6.364 6.364-1.414-1.414z"/></svg>
                                     <span class="${this.data.getVariableStyling()}">
                                        ${this.data.getVarname()}
                                     </span>
                                </div>
                            </div>
                        </div>
                        <div class="quo-dump">
                            ${this.data.getDumpHTML()}
                        </div>
                    </div>
                </div>
            </div>
        `;
        return dumpContainer;
    }
}
