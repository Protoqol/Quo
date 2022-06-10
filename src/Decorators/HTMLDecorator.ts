export abstract class HTMLDecorator {

    public element: HTMLElement;

    /**
     * Check if Element in viewport.
     *
     * @returns {boolean}
     */
    public elementInViewport(): boolean {
        return document.contains(this.element)
            && this.element.classList.contains("hidden")
            && this.element.style.display !== "none"
            && this.element.style.visibility !== "hidden";
    }

    /**
     * Append element to target content.
     *
     * @param {HTMLElement} target
     */
    public append(target: HTMLElement): void {
        return target.append(this.element);
    }

    /**
     * Prepend element to target content.
     *
     * * @param {HTMLElement} target
     */
    public prepend(target: HTMLElement): void {
        return target.prepend(this.element);
    }

    /**
     * Cloak, or hide, an element.
     *
     * @param {string} cloakClass
     * @param standsInForClass
     */
    public cloak(cloakClass: string = "hidden", standsInForClass: string = "flex") {
        if (this.elementInViewport()) {
            if (this.element.classList.contains(standsInForClass)) {
                this.element.classList.remove(cloakClass);
                this.element.classList.add(cloakClass);
            }
        }
    }

    /**
     * Uncloak, or show, an element.
     *
     * @param {string} uncloakClass
     * @param standsInForClass
     */
    public uncloak(uncloakClass: string = "flex", standsInForClass: string = "hidden") {
        if (this.elementInViewport()) {
            if (!this.element.classList.contains(standsInForClass)) {
                this.element.classList.remove(standsInForClass);
                this.element.classList.add(uncloakClass);
            }
        }
    }
}
