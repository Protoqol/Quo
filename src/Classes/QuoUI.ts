import {QuoPayload} from "./QuoPayload";

export default class QuoUI {
    public activeTab: string;

    public payloads: Array<QuoPayload>;

    public tabs: Array<string>;

    public constructor() {
        this.payloads = [];
        this.tabs = [];
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
                console.log(origin);
            });
            let tab = document.createElement("div");
            tab.id = `quo-origin-${this.tabs[0]}`;
            tab.classList.add("quo-origin-tab", "inactive-tab");
            tab.innerText = this.tabs[0];
            tabsContainer.append(tab);
        }
    }

    public filterByTab(selectedTab: string) {
        this.activeTab = selectedTab;

    }
}
