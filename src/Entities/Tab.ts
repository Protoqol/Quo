import "../Abstract/Window";
import {HTMLDecorator} from "../Decorators/HTMLDecorator";

export class Tab extends HTMLDecorator {
    /**
     * @type {HTMLElement}
     */
    public element: HTMLElement;

    /**
     * @type {string}
     */
    public tabFilter: string;

    /**
     * @type {number}
     */
    public id: number;

    /**
     * @param {string} tabFilter
     */
    constructor(tabFilter: string) {
        super();
        this.id = this.generateRepeatableHash(tabFilter);
        this.tabFilter = tabFilter;
        this.element = this.makeElement();
    }

    /**
     * Make new Tab instance.
     *
     * @param {string} tabFilter
     * @returns {Tab}
     */
    public static make(tabFilter: string): Tab {
        return new Tab(tabFilter);
    }

    /**
     * Create tab element.
     *
     * @returns {HTMLElement}
     */
    public makeElement(): HTMLElement {
        let tab = document.createElement("div");
        let closeTab = null;

        tab.id = `quo-origin-${this.tabFilter.toLowerCase()}`;

        if (this.tabFilter !== "All") {
            tab.classList.add("quo-origin-tab", "inactive-tab");
            tab.title = `Only show requests from ${this.tabFilter}`;
            tab.dataset.filter = this.tabFilter;

            const tabName = document.createElement("span");
            tabName.innerText = this.tabFilter;
            tab.append(tabName);

            closeTab = document.createElement("div");
            closeTab.id = "closeTab";
            closeTab.innerHTML = `
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="12" height="12" fill="currentColor"><path d="M12 10.586l4.95-4.95 1.414 1.414-4.95 4.95 4.95 4.95-1.414 1.414-4.95-4.95-4.95 4.95-1.414-1.414 4.95-4.95-4.95-4.95L7.05 5.636z"/></svg>        
            `;
            closeTab.title = "Close this tab";
            closeTab.addEventListener("click", (e: PointerEvent) => {
                e.stopPropagation();
                Tab.closeTab(tab);
            });

        } else {
            tab.classList.add("quo-origin-tab", "active-tab");
            tab.title = "Show all requests";
            const tabName = document.createElement("span");
            tabName.innerText = this.tabFilter;
            tab.append(tabName);
        }

        if (closeTab) {
            tab.append(closeTab);
        }

        tab.addEventListener("click", this.selectTabHandler);

        return tab;
    }

    /**
     * When clicked on tab event.
     *
     * @param {PointerEvent} e
     */
    public selectTabHandler(e: PointerEvent) {
        const target = e.currentTarget as HTMLElement;
        let filter = target.dataset.filter ?? "all";
        window.QuoState.setActiveTab(filter);
        Tab.filterPayloads(filter);
    }

    /**
     * Filter payloads for selected tab.
     */
    public static filterPayloads(filter: string) {
        // Trigger a re-search with empty input to refresh visibility based on tab
        const searchInput = document.getElementById("search") as HTMLInputElement;
        const event = new InputEvent('keyup');
        searchInput.dispatchEvent(event);
    }

    /**
     * Close tab and remove from state.
     *
     * @param {HTMLElement} tab
     */
    public static closeTab(tab: HTMLElement) {
        window.QuoState.tabs.forEach((tabInState: Tab, index: number, object: Array<Tab>) => {
            if (tabInState.element.id === tab.id) {
                object.splice(index, 1);
            }
        });
        tab.remove();
    }
}
