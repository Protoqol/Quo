import "../Abstract/Window";

export class Tab {
    private element: HTMLElement;

    private tabCategory: string;

    constructor(tabCategory: string) {
        this.tabCategory = tabCategory;
    }

    private selectTabHandler(e: PointerEvent) {
        window.QuoState.activeTab = (e.target as HTMLElement).innerText.toLowerCase();
        window.QuoState.filterByActiveTab();
    }
}
