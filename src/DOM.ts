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
        let dumpElement: HTMLElement;

        if (window.QuoState.autoGroup) {
            const existingContainer = document.querySelector(`.quo-dump-container[data-request="${payload.data.getUid()}"]`);
            if (existingContainer) {
                const dumpsList = existingContainer.querySelector(".dumps");
                const newGroupedWrapper = payload.element.querySelector(".grouped-dump-wrapper");

                if (newGroupedWrapper) {
                    newGroupedWrapper.classList.add("animate-slide-in-top");
                    dumpsList.appendChild(newGroupedWrapper);
                }

                dumpElement = existingContainer as HTMLElement;
            } else {
                dumpElement = payload.prepend(DOM.payloadsContainer);
                dumpElement.classList.add("animate-slide-in-top");
            }
        } else {
            dumpElement = payload.prepend(DOM.payloadsContainer);
            dumpElement.classList.add("animate-slide-in-top");
        }

        if (window.QuoState.activeTab.toLowerCase() !== "all" && window.QuoState.activeTab !== payload.data.getSenderOrigin()) {
            dumpElement.classList.add("hidden");
            dumpElement.classList.remove("flex");
        } else {
            dumpElement.classList.add("flex");
            dumpElement.classList.remove("hidden");
        }

        window.Sfdump(payload.data.getUuid());
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

        if (searchValue) {
            searchValue = searchValue.replace("$", "").toLowerCase();

            let count = 0;
            const allSearchable = document.querySelectorAll(`i[data-searchable]`);
            allSearchable.forEach((node: HTMLElement) => {
                if (node.dataset.searchable.toLowerCase().includes(searchValue)) {
                    count++;
                }
            });

            document.getElementById("searchResult").innerText = `Found ${count} result${count !== 1 ? "s" : ""}`;
        } else {
            document.getElementById("searchResult").innerText = ``;
        }

        const allContainers = document.querySelectorAll(`.quo-dump-container`);
        allContainers.forEach((dumpContainer: HTMLElement) => {
            const allSearchableInContainer = dumpContainer.querySelectorAll(`i[data-searchable]`);

            // Check if matches the active tab filter
            const activeTab = window.QuoState.activeTab.toLowerCase();
            const payloadOrigin = dumpContainer.dataset.domain.toLowerCase();
            const tabMatches = (activeTab === "all" || activeTab === payloadOrigin);

            if (canSearch) {
                let anyMatch = false;
                allSearchableInContainer.forEach((node: HTMLElement) => {
                    if (node.dataset.searchable.toLowerCase().includes(searchValue)) {
                        anyMatch = true;
                    }
                });

                if (anyMatch && tabMatches) {
                    dumpContainer.classList.remove("hidden");
                    dumpContainer.classList.add("flex");
                } else {
                    dumpContainer.classList.remove("flex");
                    dumpContainer.classList.add("hidden");
                }
            } else {
                if (tabMatches) {
                    dumpContainer.classList.remove("hidden");
                    dumpContainer.classList.add("flex");
                } else {
                    dumpContainer.classList.remove("flex");
                    dumpContainer.classList.add("hidden");
                }
            }
        });
    }

    /**
     * Register all DOM handlers for Quo.
     */
    public static registerHandlers() {
        document.getElementById("clearLog").addEventListener("click", DOM.clearPage);
        document.getElementById("search").addEventListener("keyup", DOM.search);
        document.getElementById("autoGroupToggle").addEventListener("change", (e: Event) => {
            window.QuoState.autoGroup = (e.target as HTMLInputElement).checked;
        });
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
