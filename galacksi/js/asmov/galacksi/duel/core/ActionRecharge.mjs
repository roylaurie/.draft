'use strict';

import Action from './Action.mjs';
import RechargeItem from './RechargeItem.mjs';
import Energy from './Energy.mjs';
import Character from './Character.mjs';

export default class ActionRecharge extends Action {
    #rechargeItem = null;
    #energyType = null;

    constructor(actor, rechargeItem, energyType) {
        super(actor);
        this.#rechargeItem = rechargeItem;
        this.#energyType = energyType;
    }

    act() {
        if (this.#rechargeItem.isExhausted()) {
            throw new Error('Item is exhausted');
        }

        this.#rechargeItem.exhaust();
        const actor = this.getActor();
        actor.rechargeEnergy(this.#energyType, 2);
        console.log(actor.getName(), 'recharged 2', Energy.names[this.#energyType]);
    }
}


