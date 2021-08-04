'use strict';

import Item from './Item.mjs';

export default class Equipment {
    static slots = {
        left:   0x00,
        right:  0x01
    };

    #items = [];

    constructor() {
        for (const slot in Equipment.slots) {
            this.#items[Equipment.slots[slot]] = null;
        }
    }

    add(slot, item) {
        if (this.#items[slot] !== null) {
            throw new Error('Item already in slot.');
        }

        this.#items[slot] = item; 
    }

    remove(slot) {
        if (this.#items[slot] === null) {
            throw new Error('No item in slot.');
        }

        const item = this.#items[slot];
        this.#items[slot] = null;
        return item;
    }

    get(slot) {
        if (this.#items[slot] === null) {
            throw new Error('No item in slot.');
        }

        return this.#items[slot];
    }

    occupied(slot) {
        return this.#items[slot] !== null;
    }

    initRound() {
        for (let i = 0; i < this.#items.length; ++i) {
            this.#items[i].restore();
        }
    }
}


