import {QuoPayload} from "./QuoPayload";
import {QuoPayloadInterface} from "../Interfaces/QuoPayloadInterface";
import "../Interfaces/Window";

export class QuoDom {

    /**
     * @type {QuoPayload}
     */
    public payload: QuoPayload;

    /**
     * @param {QuoPayload} payload
     */
    constructor(payload: QuoPayload) {
        this.payload = payload;
    }

    /**
     * Make new quo entry
     *
     * @param e
     * @param receivedPayload
     */
    public static make(e: Event, receivedPayload: QuoPayloadInterface) {
        let self = new QuoDom(QuoPayload.make(receivedPayload));
        self.hideEmptyListMessage();
        self.addHtmlToList();
        self.afterAdditionHandler();
        return self;
    }

    /**
     * Any logic that has to be run after adding a new payload.
     */
    public afterAdditionHandler(): void {
        let dump = document.querySelector(`div[id*=quo-${this.payload.getUid()}] pre[id*=quo-dump-]`);
        window.Sfdump(dump.id);
        let prev: any = null;
        Array.from(document.getElementsByTagName("article")).reverse().forEach(function (article: HTMLElement) {
            const dedupId = article.dataset.dedupId;
            if (dedupId === prev) {
                article.getElementsByTagName("header")[0].classList.add("hidden");
            }
            prev = dedupId;
        });
    }

    /**
     * Add received payload to Quo FE
     */
    public addHtmlToList(): void {
        document.getElementById("quo").prepend(this.getVarInjectedHTML());
    }

    /**
     * Hide the message that displays when no payloads are present.
     */
    public hideEmptyListMessage(): void {
        document.getElementById("quoNoRequestsMessage").classList.add("hidden");
    }

    /**
     * Create Quo HTML container containing all the variables from the payload.
     */
    public getVarInjectedHTML(): HTMLElement {
        let dumpContainer = document.createElement("div");
        dumpContainer.classList.add("quo-dump-container");
        dumpContainer.id = `quo-${this.payload.getUid()}`;

        dumpContainer.innerHTML = `
            <div class="time">
                <span>${this.payload.getOrigin()}</span>
                <span>${this.payload.getSenderOrigin()} - ${this.payload.getTime()}</span>
            </div>
            <div class="quo-actual-dump">
                <h3 class="quo-title">
                    <div class="file">
                        <div>
                            <span class="received">Received (arg #${this.payload.getNoOfNodes() + 1} of ${this.payload.getAllPassedVariables().length})</span>
                            <div class="passed" style="${this.payload.getVarname ? "" : "display:none;"}">
                                <div>
                                     <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M13.172 12l-4.95-4.95 1.414-1.414L16 12l-6.364 6.364-1.414-1.414z"/></svg>
                                     <span class="${this.payload.getVariableStyling()}">
                                        ${this.payload.getVarname()}
                                     </span>
                                </div> 
                            </div>
                       </div>  
                       <div style="margin-top:.75rem;">
                           <span class="received">Origin</span>
                           <div class="passed">
                               <div>
                                   <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="18" height="18"><path fill="none" d="M0 0h24v24H0z"/><path d="M7.784 14l.42-4H4V8h4.415l.525-5h2.011l-.525 5h3.989l.525-5h2.011l-.525 5H20v2h-3.784l-.42 4H20v2h-4.415l-.525 5h-2.011l.525-5H9.585l-.525 5H7.049l.525-5H4v-2h3.784zm2.011 0h3.99l.42-4h-3.99l-.42 4z"/></svg>
                                   <span>${this.payload.getSenderOrigin()}</span>
                               </div>
                           </div>
                       </div>
                    </div>
                </h3>
                <div class="dumps">
                    ${this.payload.getDumpHTML()}
                </div>
            </div>
        `;

        return dumpContainer;
    }

    /**
     * Clear page of payloads.
     */
    public static clearPage(): void {
        let nodes: any = document.querySelectorAll("[class=quo-dump-container]");

        for (let container of nodes) {
            container.remove();
        }

        if (document.getElementById("quoNoRequestsMessage").classList.contains("hidden")) {
            document.getElementById("quoNoRequestsMessage").classList.remove("hidden");
        }
    }

    /**
     * Search for payloads by value.
     *
     * @param e
     */
    public static search(e: any) {
        let searchValue: string = e.target.value;
        let canSearch: boolean = Boolean(searchValue);
        let resultNodes: any = null;
        let allNodes: NodeList = document.querySelectorAll(`i[data-searchable]`);

        if (Boolean(searchValue)) {
            searchValue = searchValue.replace("$", "");
            resultNodes = document.querySelectorAll(`i[data-searchable*='${searchValue}']`);
            document.getElementById("searchResult").innerText = `Found ${resultNodes.length} result${resultNodes.length > 1 || resultNodes.length === 0 ? "s" : ""}`;
        } else {
            document.getElementById("searchResult").innerText = ``;
        }

        allNodes.forEach((node: HTMLElement) => {
            let canStay = true;
            let searchable = node.dataset.searchable;
            let dumpContainer = node.parentElement.parentElement.parentElement.parentElement;

            if (canSearch) {
                resultNodes.forEach((node: HTMLElement) => {
                    if (searchable === node.dataset.searchable) {
                        canStay = false;
                    }
                });
            } else {
                dumpContainer.style.display = "";
                return true;
            }


            if (canStay) {
                dumpContainer.style.display = "none";
            } else {
                dumpContainer.style.display = "";
            }
        });
    }

    /**
     * Register all DOM handlers for Quo.
     */
    public static registerHandlers() {
        document.getElementById("clearLog").addEventListener("click", QuoDom.clearPage);
        document.getElementById("search").addEventListener("keyup", QuoDom.search);
    }
}
