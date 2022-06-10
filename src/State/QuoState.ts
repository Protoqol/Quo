import Payload from "../Entities/Payload";
import "../Abstract/Window";

/**
 * This class will be instantiated once and is kept in window.QuoState.
 */
export default class QuoState {

    public activeTab: string;

    public payloads: Array<Payload>;

    public tabs: Array<string>;

    constructor() {
        this.payloads = [];
        this.tabs = ["All"];
    }

    /**
     * Save payload to QuoUI state.
     *
     * @param {Payload} payload
     */
    public savePayloadToState(payload: Payload): void {
        this.payloads.push(payload);
        this.addCategoryToTabs(payload.data.getSenderOrigin());
    }

    /**
     * Add new category to tabs state.
     *
     * @param {string} origin
     */
    public addCategoryToTabs(origin: string): void {
        if (!this.tabs.includes(origin)) {
            this.tabs.push(origin);
            this.updateTabs();
        }
    }

    /**
     * Check if new tabs should be created or removed.
     */
    public updateTabs() {
        const tabsContainer = document.querySelector(".quo-origin-tabs");

        this.tabs.forEach((origin: string) => {
            if (!QuoState.tabExists(origin)) {
                const tab = document.createElement("div");
                tab.id = `quo-origin-${origin}`;
                if (origin !== "All") {
                    tab.classList.add("quo-origin-tab", "inactive-tab");
                } else {
                    tab.classList.add("quo-origin-tab", "active-tab");
                }
                tab.dataset.origin = origin;
                tab.innerText = origin;
                tab.addEventListener("click", this.selectTabEventHandler);
                tabsContainer.append(tab);
            }
        });
    }

    /**
     * Filter payloads by tab. Only fired when clicked. Active filtering is done in Payload.
     */
    public filterByActiveTab() {
        QuoState.setTabActive(this.activeTab);

        this.payloads.forEach((payload: Payload) => {
            payload.uncloak();
        });

        if (this.activeTab.toLowerCase() === "all") {
            return true;
        }

        this.payloads.forEach((payload: Payload) => {
            if (payload.data.getSenderOrigin() !== this.activeTab) {
                console.log("Cloaking " + payload.data.getSenderOrigin());
                payload.cloak();
            }
        });
    }

    /**
     * When clicked on tab this function will fire.
     *
     * @param e
     */
    public selectTabEventHandler(e: PointerEvent) {
        window.QuoState.activeTab = (e.target as HTMLElement).innerText.toLowerCase();
        window.QuoState.filterByActiveTab();
    }

    /**
     * Set tab to active state. UI and filtering.
     *
     * @param {string} originTab
     * @private
     */
    private static setTabActive(originTab: string) {
        const tabs = document.querySelectorAll(".quo-origin-tab");
        tabs.forEach((tab) => {
            if (tab.classList.contains("active-tab")) {
                tab.classList.remove("active-tab");
            }

            if (tab.classList.contains("inactive-tab")) {
                tab.classList.remove("inactive-tab");
            }

            if (tab.innerHTML.trim() === originTab) {
                tab.classList.add("active-tab");
            } else {
                tab.classList.add("inactive-tab");
            }
        });
    }

    /**
     * @param {string} originTab
     * @returns {HTMLElement}
     * @private
     */
    private static tabExists(originTab: string) {
        return document.getElementById(`quo-origin-${originTab}`);
    }
}
