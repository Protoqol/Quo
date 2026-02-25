export abstract class HTMLDecorator {

    /**
     * @type {HTMLElement}
     */
    public element: HTMLElement;

    /**
     * @type {boolean}
     */
    public isCloaked: boolean;

    /**
     * @type {boolean}
     */
    public isUncloaked: boolean;

    /**
     * Check if Element in viewport.
     *
     * @returns {boolean}
     */
    public elementInViewport(): boolean {
        return this.element.classList.contains("hidden")
            && this.element.style.display !== "none"
            && this.element.style.visibility !== "hidden";
    }

    /**
     * Append element to target content.
     *
     * @param {HTMLElement} target
     */
    public append(target: HTMLElement): HTMLElement {
        target.append(this.element);
        return this.element;
    }

    /**
     * Prepend element to target content.
     *
     * * @param {HTMLElement} target
     */
    public prepend(target: HTMLElement): HTMLElement {
        target.prepend(this.element);
        return this.element;
    }

    /**
     * Cloak, or hide, an element.
     *
     * @param {string} cloakClass
     * @param standsInForClass
     */
    public cloak(cloakClass: string = "hidden", standsInForClass: string = "flex") {
        if (this.element.classList.contains(standsInForClass)) {
            this.element.classList.remove(standsInForClass);
            this.element.classList.add(cloakClass);

            this.isCloaked = true;
            this.isUncloaked = false;
        }
    }

    /**
     * Uncloak, or show, an element.
     *
     * @param {string} uncloakClass
     * @param standsInForClass
     */
    public uncloak(uncloakClass: string = "flex", standsInForClass: string = "hidden") {
        if (this.element.classList.contains(standsInForClass)) {
            this.element.classList.remove(standsInForClass);
            this.element.classList.add(uncloakClass);
            this.isCloaked = false;
            this.isUncloaked = true;
        }
    }

    /**
     * Generate a hash that produces a consistent hash with the same value.
     *
     * @param {string} str
     * @param {number} seed
     * @returns {number}
     */
    public generateRepeatableHash(str: string, seed: number = 0) {
        str = str.toLowerCase();

        let h1 = 0xdeadbeef ^ seed, h2 = 0x41c6ce57 ^ seed;

        for (let i = 0, ch; i < str.length; i++) {
            ch = str.charCodeAt(i);
            h1 = Math.imul(h1 ^ ch, 2654435761);
            h2 = Math.imul(h2 ^ ch, 1597334677);
        }

        h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
        h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);

        return 4294967296 * (2097151 & h2) + (h1 >>> 0);
    }

}
