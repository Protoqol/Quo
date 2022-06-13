import {IncomingPayloadInterface} from "../../Abstract/IncomingPayloadInterface";
import {v4 as uuid} from "uuid";
import "../../Abstract/Window";

export default class PayloadData {

    /**
     * @type {IncomingPayloadInterface}
     * @private
     */
    private payload: IncomingPayloadInterface;

    /**
     * Unique identifier for element, not to be confused with UID which is used to identify incoming calls.
     * @type {string}
     * @private
     */
    private readonly uuid: string;

    /**
     * @param {IncomingPayloadInterface} payload
     */
    constructor(payload: IncomingPayloadInterface) {
        this.payload = payload;
        this.uuid = uuid();
    }

    /**
     * @returns {string}
     */
    public getUuid() {
        return this.uuid;
    }

    /**
     * @returns {number}
     */
    public getId(): number {
        return this.payload.meta.id;
    }

    /**
     * @returns {string}
     */
    public getUid(): string {
        return this.payload.meta.uid;
    }

    /**
     * @returns {string}
     */
    public getOrigin(): string {
        return this.payload.meta.origin;
    }

    /**
     * @returns {string}
     */
    public getOriginWithoutLineNr(): string {
        const lastIndex = this.payload.meta.origin.lastIndexOf(":");

        return this.payload.meta.origin.substring(0, lastIndex);
    }

    /**
     * Get line number.
     *
     * @returns {string}
     */
    public getLineNr(): string {
        const lastIndex = this.payload.meta.origin.lastIndexOf(":");
        return this.payload.meta.origin.substring(lastIndex + 1);
    }

    /**
     * @returns {string}
     */
    public getSenderOrigin(): string {
        return this.payload.meta.senderOrigin;
    }

    /**
     * @returns {string}
     */
    public getTime(): string {
        return this.payload.meta.time;
    }

    /**
     * @returns {string}
     */
    public getDumpHTML(): string {
        return this.payload.data;
    }

    /**
     * @returns {Array<string>}
     */
    public getAllPassedVariables(): Array<string> {
        const variables = this.payload.meta.calledVariable.split(",");

        if (this.getId() > 0) {
            return variables.reverse();
        } else {
            return [variables[0]];
        }
    }

    /**
     * @returns {string}
     */
    public getCurrentVariableName(): string {
        return this.getAllPassedVariables()[this.getNoOfNodes()];
    }

    /**
     * @returns {string}
     */
    public getVariableStyling(): string {
        switch (true) {
            case this.getCurrentVariableName().includes("$"):
                return "var-style";
            case this.getCurrentVariableName().includes("()") && !this.getCurrentVariableName().includes("::"):
                return "func-style";
            case this.getCurrentVariableName().includes("::") || this.getCurrentVariableName().includes("new "):
                return "class-style";
            case this.getCurrentVariableName().includes("[") || this.getCurrentVariableName().includes("]"):
                return "array-style";
            default:
                return "def-style";
        }
    }

    /**
     * @returns {string}
     */
    public getVarname() {
        return this.getCurrentVariableName()
                   .replace(/&/g, "&amp;")
                   .replace(/</g, "&lt;")
                   .replace(/>/g, "&gt;")
                   .replace(/"/g, "&quot;")
                   .replace(/'/g, "&#039;");
    }

    /**
     * @returns {number}
     */
    public getNoOfNodes(): number {
        if (this.getId() > 0) {
            return document.querySelectorAll(`div[data-request='${this.getUid()}']`).length;
        }

        return 0;
    }
}
