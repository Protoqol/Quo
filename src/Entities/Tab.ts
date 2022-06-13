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
            closeTab = document.createElement("div");
            closeTab.innerHTML = `
                <svg id="closeTab" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="12" height="12"><path fill="none" d="M0 0h24v24H0z"/><path d="M12 10.586l4.95-4.95 1.414 1.414-4.95 4.95 4.95 4.95-1.414 1.414-4.95-4.95-4.95 4.95-1.414-1.414 4.95-4.95-4.95-4.95L7.05 5.636z"/></svg>        
            `;
            closeTab.title = "Close this tab";
            closeTab.addEventListener("click", (e: PointerEvent) => {
                Tab.closeTab(tab);
            });

        } else {
            tab.classList.add("quo-origin-tab", "active-tab");
            tab.title = "Show all requests";
        }

        tab.innerHTML = `
            ${this.tabFilter} 
        `;

        if (closeTab) {
            tab.append(closeTab);
        } else {
            tab.classList.add("pr-2");
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
        let filter = (e.target as HTMLElement).dataset.filter ?? "all";
        window.QuoState.setActiveTab(filter);
        Tab.filterPayloads(filter);
    }

    /**
     * Filter payloads for selected tab.
     */
    public static filterPayloads(filter: string) {
        window.QuoState.payloads.forEach(payload => {
            payload.uncloak();

            if (filter === "all") {
                return;
            }

            if (payload.data.getSenderOrigin().toLowerCase() !== filter.toLowerCase()) {
                payload.cloak();
            } else {
                payload.uncloak();
            }
        });
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
