import {IncomingPayloadInterface} from "../../Abstract/IncomingPayloadInterface";
import {v4 as uuid} from "uuid";
import "../../Abstract/Window";

export default class PayloadData {

    /**
     * @type {IncomingPayloadInterface}
     * @private
     */
    private payload: IncomingPayloadInterface;

    /**
     * Unique identifier for element, not to be confused with UID which is used to identify incoming calls.
     * @type {string}
     * @private
     */
    private uuid: string;

    /**
     * @param {IncomingPayloadInterface} payload
     */
    constructor(payload: IncomingPayloadInterface) {
        this.payload = payload;
        this.uuid = uuid();
    }

    /**
     * Create Quo HTML container containing all the variables from the payload.
     */
    public getVarInjectedHTML(): HTMLElement {
        const dumpContainer = document.createElement("div");
        dumpContainer.id = `quo-${this.getUid()}`;
        dumpContainer.classList.add("quo-dump-container");
        dumpContainer.innerHTML = `
            <div class="time">
                <a id="quo-link-${this.getUid()}" target="_top" href="phpstorm://open?file=${this.getOriginWithoutLineNr()}&line=${this.getLineNr()}">${this.getOrigin()}</a>
                <span>${this.getSenderOrigin()} - ${this.getTime()}</span>
            </div>
            <div class="quo-actual-dump">
                <h3 class="quo-title">
                    <div class="file">
                        <div>
                            <span class="received">Received (arg #${this.getNoOfNodes() + 1} of ${this.getAllPassedVariables().length})</span>
                            <div class="passed" style="${this.getVarname ? "" : "display:none;"}">
                                <div>
                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M13.172 12l-4.95-4.95 1.414-1.414L16 12l-6.364 6.364-1.414-1.414z"/></svg>
                                     <span class="${this.getVariableStyling()}">
                                        ${this.getVarname()}
                                     </span>
                                </div> 
                            </div>
                       </div>  
                       <div style="margin-top:.75rem;">
                           <span class="received">Origin</span>
                           <div class="passed">
                               <div>
                                   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M7.784 14l.42-4H4V8h4.415l.525-5h2.011l-.525 5h3.989l.525-5h2.011l-.525 5H20v2h-3.784l-.42 4H20v2h-4.415l-.525 5h-2.011l.525-5H9.585l-.525 5H7.049l.525-5H4v-2h3.784zm2.011 0h3.99l.42-4h-3.99l-.42 4z"/></svg>
                                   <span>${this.getSenderOrigin()}</span>
                               </div>
                           </div>
                       </div>
                    </div>
                </h3>
                <div class="dumps">
                    ${this.getDumpHTML()}
                </div>
            </div>
        `;

        return dumpContainer;
    }

    /**
     * @returns {number}
     */
    public getId(): number {
        return this.payload.meta.id;
    }

    /**
     * @returns {string}
     */
    public getUid(): string {
        return this.payload.meta.uid;
    }

    /**
     * @returns {string}
     */
    public getOrigin(): string {
        return this.payload.meta.origin;
    }

    /**
     * @returns {string}
     */
    public getOriginWithoutLineNr(): string {
        const lastIndex = this.payload.meta.origin.lastIndexOf(":");

        return this.payload.meta.origin.substring(0, lastIndex);
    }

    /**
     * Get line number.
     *
     * @returns {string}
     */
    public getLineNr(): string {
        const lastIndex = this.payload.meta.origin.lastIndexOf(":");
        return this.payload.meta.origin.substring(lastIndex + 1);
    }

    /**
     * @returns {string}
     */
    public getSenderOrigin(): string {
        return this.payload.meta.senderOrigin;
    }

    /**
     * @returns {string}
     */
    public getTime(): string {
        return this.payload.meta.time;
    }

    /**
     * @returns {string}
     */
    public getDumpHTML(): string {
        return this.payload.data;
    }

    /**
     * @returns {Array<string>}
     */
    public getAllPassedVariables(): Array<string> {
        const variables = this.payload.meta.calledVariable.split(",");

        if (this.getId() > 0) {
            return variables.reverse();
        } else {
            return [variables[0]];
        }
    }

    /**
     * @returns {string}
     */
    public getCurrentVariableName(): string {
        return this.getAllPassedVariables()[this.getNoOfNodes()];
    }

    /**
     * @returns {string}
     */
    public getVariableStyling(): string {
        switch (true) {
            case this.getCurrentVariableName().includes("$"):
                return "var-style";
            case this.getCurrentVariableName().includes("()") && !this.getCurrentVariableName().includes("::"):
                return "func-style";
            case this.getCurrentVariableName().includes("::") || this.getCurrentVariableName().includes("new "):
                return "class-style";
            case this.getCurrentVariableName().includes("[") || this.getCurrentVariableName().includes("]"):
                return "array-style";
            default:
                return "def-style";
        }
    }

    /**
     * @returns {string}
     */
    public getVarname() {
        return this.getCurrentVariableName()
                   .replace(/&/g, "&amp;")
                   .replace(/</g, "&lt;")
                   .replace(/>/g, "&gt;")
                   .replace(/"/g, "&quot;")
                   .replace(/'/g, "&#039;");
    }

    /**
     * @returns {number}
     */
    public getNoOfNodes(): number {
        if (this.getId() > 0) {
            return document.querySelectorAll(`div[id=quo-${this.getUid()}]`).length;
        }

        return 0;
    }
}
