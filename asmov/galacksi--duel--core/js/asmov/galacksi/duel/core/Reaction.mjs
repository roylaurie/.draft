'use strict';

export default class Reaction {
    #action = null;
    #reactor = null;

    constructor(action, reactor) {
        this.#action = action;
        this.#reactor = reactor;
    }

    getReactor() {
        return this.#reactor;
    }
    
    getAction() {
        return this.#action;
    }

    react() {
    }
}


