import {QuoPayloadInterface} from "../Interfaces/QuoPayloadInterface";

export class QuoPayload {

    /**
     * @type {QuoPayloadInterface}
     * @private
     */
    private payload: QuoPayloadInterface;

    /**
     * @param {QuoPayloadInterface} payload
     */
    constructor(payload: QuoPayloadInterface) {
        this.payload = payload;
    }

    /**
     * @param {QuoPayloadInterface} payload
     * @returns {QuoPayload}
     */
    public static make(payload: QuoPayloadInterface) {
        return new QuoPayload(payload);
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
        let variables = this.payload.meta.calledVariable.split(",");

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
                return "";
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
            return document.querySelectorAll(`div[id=quo-${this.getUid()}]`).length;
        }

        return 0;
    }
}
