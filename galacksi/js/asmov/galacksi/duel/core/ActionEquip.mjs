'use strict';

import Action from './Action.mjs';
import Character from './Character.mjs';
import Inventory from './Inventory.mjs';
import Equipment from './Equipment.mjs';

export default class ActionEquip extends Action {
    #equipmentSlot = null;
    #inventoryItem = null;

    constructor(actor, equipmentSlot, inventoryItem) {
        super(actor);
        this.#equipmentSlot = equipmentSlot;
        this.#inventoryItem = inventoryItem;
    }

    act() {
        super.act();
        const actor = this.getActor();
        const equipment = actor.equipment();
        const inventory = actor.inventory();

        if (equipment.occupied(this.#equipmentSlot)) {
            const equippedItem = equipment.remove(this.#equipmentSlot);
            inventory.add(equippedItem);
        }
        
        inventory.remove(this.#inventoryItem);
        equipment.add(this.#equipmentSlot, this.#inventoryItem);

        console.log(actor.getName(), 'wielded', this.#inventoryItem.getName());
    }
}


