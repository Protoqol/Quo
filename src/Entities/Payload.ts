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
                <a id="quo-link-${this.data.getUid()}" target="_top" href="phpstorm://open?file=${this.data.getOriginWithoutLineNr()}&line=${this.data.getLineNr()}">${this.data.getOrigin()}</a>
                <span>${this.data.getSenderOrigin()} - ${this.data.getTime()}</span>
            </div>
            <div class="quo-actual-dump">
                <h3 class="quo-title">
                    <div class="file">
                        <div>
                            <span class="received">Received (arg #${this.data.getNoOfNodes() + 1} of ${this.data.getAllPassedVariables().length})</span>
                            <div class="passed" style="${this.data.getVarname ? "" : "display:none;"}">
                                <div>
                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M13.172 12l-4.95-4.95 1.414-1.414L16 12l-6.364 6.364-1.414-1.414z"/></svg>
                                     <span class="${this.data.getVariableStyling()}">
                                        ${this.data.getVarname()}
                                     </span>
                                </div> 
                            </div>
                       </div>  
                       <div style="margin-top:.75rem;">
                           <span class="received">Origin</span>
                           <div class="passed">
                               <div>
                                   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M7.784 14l.42-4H4V8h4.415l.525-5h2.011l-.525 5h3.989l.525-5h2.011l-.525 5H20v2h-3.784l-.42 4H20v2h-4.415l-.525 5h-2.011l.525-5H9.585l-.525 5H7.049l.525-5H4v-2h3.784zm2.011 0h3.99l.42-4h-3.99l-.42 4z"/></svg>
                                   <span>${this.data.getSenderOrigin()}</span>
                               </div>
                           </div>
                       </div>
                    </div>
                </h3>
                <div class="dumps">
                    ${this.data.getDumpHTML()}
                </div>
            </div>
        `;
        return dumpContainer;
    }
}
