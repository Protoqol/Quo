import Payload from "../Entities/Payload";
import "../Abstract/Window";
import {Tab} from "../Entities/Tab";

/**
 * This class will be instantiated once and is kept in window.QuoState.
 */
export default class QuoState {

    /**
     * @type string
     */
    public activeTab: string;

    /**
     * @type {Array<Payload>}
     */
    public payloads: Array<Payload>;

    /**
     * @type {Array<Tab>}
     */
    public tabs: Array<Tab>;

    /**
     * @type {boolean}
     */
    public autoGroup: boolean = true;

    /**
     * @type {string[]}
     * @private
     */
    private static defaultTabs = [
        "All",
    ];

    constructor() {
        this.payloads = [];
        this.tabs = [];

        QuoState.defaultTabs.forEach((tab, index) => {
            let tabElement = Tab.make(tab);
            this.admitTabFilterToState(tabElement);
            if (index === 0) {
                this.setActiveTab(tabElement.tabFilter);
            }
        });
    }

    /**
     * Save payload to QuoUI state.
     *
     * @param {Payload} payload
     */
    public admitPayloadToState(payload: Payload): void {
        this.payloads.push(payload);
        this.admitTabFilterToState(Tab.make(payload.data.getSenderOrigin()));
    }

    /**
     * Add new category to tabs state.
     *
     * @param tab
     */
    public admitTabFilterToState(tab: Tab): void {
        if (!this.tabs.some(stateTab => stateTab.tabFilter === tab.tabFilter)) {
            this.tabs.push(tab);
            tab.append(document.getElementById("quo-tabs-container"));
        }
    }

    /**
     * @param {Tab} tab
     */
    public setActiveTab(tab: string): void {
        this.tabs.forEach(tabInState => {
            if (tabInState.tabFilter.toLowerCase() === tab.toLowerCase()) {
                this.activeTab = tab;
                tabInState.uncloak("active-tab", "inactive-tab");
            }else{
                tabInState.cloak("inactive-tab", "active-tab");
            }
        });
    }

    /**
     * @returns {Tab}
     */
    public getActiveTab(): string {
        return this.activeTab;
    }
}
