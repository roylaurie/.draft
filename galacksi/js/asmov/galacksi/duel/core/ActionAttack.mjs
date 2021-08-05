'use strict';

import Action from './Action.mjs';
import Character from './Character.mjs';
import Reaction from './Reaction.mjs';
import WeaponItem from './WeaponItem.mjs';
import Energy from './Energy.mjs';

export default class ActionAttack extends Action {
    #targetCharacter = null;
    #weaponItem = null;
    #energyType = null;

    constructor(actor, targetCharacter, weaponItem, energyType) {
        super(actor);

        this.#targetCharacter = targetCharacter;
        this.#weaponItem = weaponItem;
        this.#energyType = energyType;
    }

    getWeaponItem() {
        return this.#weaponItem;
    }

    getEnergyType() {
        return this.#energyType;
    }

    act() {
        super.act();
        const actor = this.getActor();

        actor.consumeAction();
        this.#weaponItem.exhaust();
        actor.consumeEnergy(this.#energyType, 1);

        console.log(actor.getName(), 'fires', this.#weaponItem.getName(), 'at', this.#targetCharacter.getName(), 'with', Energy.names[this.#energyType], 'energy');

        const targetReaction = this.#targetCharacter.inputReaction(this);
        targetReaction.react();
    }
}


