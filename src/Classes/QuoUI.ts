import {QuoPayload} from "./QuoPayload";

export default class QuoUI {
    public activeTab: string;

    public payloads: Array<QuoPayload>;

    public tabs: Array<string>;

    public constructor() {
        this.payloads = [];
        this.tabs = ["All"];
    }

    public savePayloadToState(payload: QuoPayload): void {
        this.payloads.push(payload);
        this.addCategoryToTabs(payload.getSenderOrigin());
    }

    public addCategoryToTabs(origin: string): void {
        if (!this.tabs.includes(origin)) {
            this.tabs.push(origin);
            this.updateTabs();
        }
    }

    public updateTabs() {
        let tabsContainer = document.querySelector(".quo-origin-tabs");
        let tabs = document.querySelectorAll("div[id*=quo-origin-tab]");

        if (tabs.length) {
            // for (let tab of container) {
            // Update tabs
            // }
        } else {
            this.tabs.forEach((origin: string) => {
                if (!QuoUI.tabExists(origin)) {
                    let tab = document.createElement("div");
                    tab.id = `quo-origin-${origin}`;
                    tab.classList.add("quo-origin-tab", "inactive-tab");
                    tab.innerText = origin;
                    tab.addEventListener("click", this.selectTabEventHandler);
                    tabsContainer.append(tab);
                }
            });
        }
    }

    public filterByActiveTab() {
        QuoUI.setTabActive(this.activeTab);

        this.payloads.forEach((payload: QuoPayload) => {
            if (this.activeTab === "all") {
                payload.uncloakPayload();
                return true;
            }

            if (payload.getSenderOrigin() !== this.activeTab) {
                payload.cloakPayload();
            } else {
                payload.uncloakPayload();
            }
        });
    }

    public selectTabEventHandler(e: any) {
        window.UI.activeTab = e.target.innerText.toLowerCase();
        window.UI.filterByActiveTab();
    }

    private static setTabActive(originTab: string) {
        let tabs = document.querySelectorAll(".quo-origin-tab");
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
