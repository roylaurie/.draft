'use strict';

import Item from './Item.mjs';
import ItemTemplate from './ItemTemplate.mjs';
import EnergyRechargerItem from './EnergyRechargerItem.mjs';

export default class Inventory {
    #items = [];

    add(item) {
        this.#items.push(item);
    }

    remove(item) {
        const i = this.#items.indexOf(item);
        this.#items.splice(i, 1);
        return item;
    }

    items() {
        return this.#items;
    }

    findByClass(itemClass) {
        return this.#items.find(item => item instanceof itemClass);
    }
}


