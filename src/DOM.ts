import Payload from "./Entities/Payload";
import "./Abstract/Window";

export default class DOM {

    /**
     * @type {HTMLElement}
     */
    public static payloadsContainer: HTMLElement = document.getElementById("quo");

    /**
     * Add received payload to front-end.
     */
    public static addHtmlToList(payload: Payload): void {
        let dump = payload.prepend(DOM.payloadsContainer);

        if (window.QuoState.activeTab.toLowerCase() !== "all" && window.QuoState.activeTab !== payload.data.getSenderOrigin()) {
            payload.cloak();
        } else {
            payload.uncloak();
        }

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
     * Hide the message that displays when no payloads are present.
     */
    public static hideEmptyListMessage(): void {
        document.getElementById("quoNoRequestsMessage").classList.add("hidden");
    }

    /**
     * Clear page of payloads.
     */
    public static clearPage(): void {
        const nodes: NodeList = document.querySelectorAll(".quo-dump-container, .quo-origin-tab[data-filter]");

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
        document.addEventListener("keydown", (e: KeyboardEvent) => {
            switch (e.key) {
                case "/":
                    e.preventDefault();
                    e.stopPropagation();
                    e.stopImmediatePropagation();
                    document.getElementById("search").focus();
            }
        });
    }
}
