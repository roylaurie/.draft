'use strict';

export default class Item {
    #name = null;
    #exhausted = false;

    name = null;

    constructor(name) {
        this.#name = name;
        this.name = name;
    }

    getName() {
        return this.#name;
    }

    exhaust() {
        this.#exhausted = true;
    }

    restore() {
        this.#exhausted = false;
    }

    isExhausted() {
        return this.#exhausted;
    }
}

 
