'use strict';

import Item from './Item.mjs';
import WeaponItem from './WeaponItem.mjs';
import RechargeItem from './RechargeItem.mjs';
import EnergyRechargerItem from './EnergyRechargerItem.mjs';

export default class ItemTemplate {
    static weapons = {
        pistol: new ItemTemplate(WeaponItem, 'pistol')
    };

    static rechargers = {
        energy: new ItemTemplate(EnergyRechargerItem, 'energy recharger'),
        omega: new ItemTemplate(RechargeItem, 'omega recharger')
    };

    #itemClass = null;
    #name = null;

    constructor(itemClass, name) {
        this.#itemClass = itemClass;
        this.#name = name;
    }

    getName() {
        return this.#name;
    }

    getItemClass() {
        return this.#itemClass;
    }

    create() {
        switch(this.#itemClass) {
        case WeaponItem:
            return new WeaponItem(this.#name);
        case EnergyRechargerItem:
            return new EnergyRechargerItem(this.#name);
        default:
            return new Item(this.#name);
        }
    }
}

 
