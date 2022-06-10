import Payload from "./Entities/Payload";
import {IncomingPayloadInterface} from "./Abstract/IncomingPayloadInterface";
import "./Abstract/Window";

export default class DOM {

    /**
     * @type Payload
     */
    public payload: Payload;

    /**
     * @param payload
     */
    constructor(payload: Payload) {
        this.payload = payload;
    }

    /**
     * Any logic that has to be run after adding a new payload.
     */
    public static afterAdditionHandler(payload: Payload): void {
        const dump = document.querySelector(`div[id*=quo-${payload.data.getUid()}] pre[id*=quo-dump-]`);
        window.Sfdump(dump.id);
        let prev: string = null;
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
    public static addHtmlToList(payload: Payload): void {
        document.getElementById("quo").prepend(payload.data.getVarInjectedHTML());
        document.getElementById(`quo-link-${payload.data.getUid()}`)
                .addEventListener("click", (e: MouseEvent) => {
                    window.MainProcess.openUrl((e.target as HTMLAnchorElement).href);
                });
    }

    /**
     * Hide the message that displays when no payloads are present.
     */
    public static hideEmptyListMessage(): void {
        document.getElementById("quoNoRequestsMessage").classList.add("hidden");
    }

    /**
     * Clear page of payloads.
     */
    public static clearPage(): void {
        const nodes: NodeList = document.querySelectorAll("[class=quo-dump-container]");

        nodes.forEach((payload: HTMLElement) => {
            payload.remove();
        });

        if (document.getElementById("quoNoRequestsMessage").classList.contains("hidden")) {
            document.getElementById("quoNoRequestsMessage").classList.remove("hidden");
        }
    }

    /**
     * Search for payloads by value.
     *
     * @param e
     */
    public static search(e: InputEvent) {
        let searchValue: string = (e.target as HTMLInputElement).value;

        const canSearch = Boolean(searchValue);
        let resultNodes: NodeList = null;

        const allNodes: NodeList = document.querySelectorAll(`i[data-searchable]`);

        if (searchValue) {
            searchValue = searchValue.replace("$", "");
            resultNodes = document.querySelectorAll(`i[data-searchable*='${searchValue}']`);
            document.getElementById("searchResult").innerText = `Found ${resultNodes.length} result${resultNodes.length > 1 || resultNodes.length === 0 ? "s" : ""}`;
        } else {
            document.getElementById("searchResult").innerText = ``;
        }

        allNodes.forEach((node: HTMLElement) => {
            let canStay = true;
            const searchable = node.dataset.searchable;
            const dumpContainer = node.parentElement.parentElement.parentElement.parentElement;

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
        document.getElementById("clearLog").addEventListener("click", DOM.clearPage);
        document.getElementById("search").addEventListener("keyup", DOM.search);
    }
}
