'use strict';

import Character from './Character.mjs';

export default class Action {
    #actor = null;

    constructor(actor) {
        this.#actor = actor;
    }

    getActor() {
        return this.#actor;
    }

    act() {
        return; // override
    }
}


