'use strict';

import Item from './Item.mjs';
import Energy from './Energy.mjs';
import WeaponItem from './WeaponItem.mjs';
import RechargeItem from './RechargeItem.mjs';
import ItemTemplate from './ItemTemplate.mjs';
import Inventory from './Inventory.mjs';
import Equipment from './Equipment.mjs';
import ActionAttack from './ActionAttack.mjs';
import ActionEquip from './ActionEquip.mjs';
import ActionRecharge from './ActionRecharge.mjs';
import ActionPass from './ActionPass.mjs';
import ReactionShield from './ReactionShield.mjs';
import ReactionPass from './ReactionPass.mjs';
import Shuffle from './Shuffle.mjs';

export default class Character {
    #name = null;
    #omegaPoints = 4;
    #numActions = 2;
    #energy = {};
    #inventory = new Inventory();
    #equipment = new Equipment();
    #targetCharacter = null;

    constructor(name) {
        this.#name = name;

        this.#energy[Energy.types.gamma] = 6;
        this.#energy[Energy.types.chi] = 6;

        this.#equipment.add(Equipment.slots.left, ItemTemplate.weapons.pistol.create());
        this.#equipment.add(Equipment.slots.right, ItemTemplate.weapons.pistol.create());

        this.#inventory.add(ItemTemplate.rechargers.energy.create());
        this.#inventory.add(ItemTemplate.rechargers.omega.create());
    }

    getName() {
        return this.#name;
    }

    getOmegaPoints() {
        return this.#omegaPoints;
    }

    getEnergy(energyType) {
        return this.#energy[energyType];
    }

    hasEnergy(energyType, minimum) {
        return this.#energy[energyType] >= minimum;
    }

    consumeEnergy(energyType, amount) {
        if (!this.hasEnergy(energyType, amount)) {
            throw new Error('Not enough energy.');
        }

        this.#energy[energyType] -= amount;
    }

    rechargeEnergy(energyType, amount) {
        this.#energy[energyType] += amount;
    }

    setTarget(character) {
        this.#targetCharacter = character;
    }

    getTarget() {
        return this.#targetCharacter;
    }

    consumeAction() {
        this.#numActions--;
    }

    initRound(round) {
        this.#numActions = 2;
        this.#equipment.initRound();
    }

    damage(amount) {
        if (this.unconscious()) {
            throw new Error('Character is already unconscious.');
        }

        this.#omegaPoints = ( this.#omegaPoints - amount < 0 ? 0 : this.#omegaPoints - amount );
    }

    conscious() {
        return this.#omegaPoints > 0;
    }

    unconscious() {
        return this.#omegaPoints < 1;
    }

    inventory() {
        return this.#inventory;
    }

    equipment() {
        return this.#equipment;
    }

    inputAction() { return; /* override */ }

    inputReaction(action) { return; /* override */ }
}

 
