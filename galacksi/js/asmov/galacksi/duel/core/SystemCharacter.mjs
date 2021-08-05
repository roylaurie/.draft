'use stict';

import Character from './Character.mjs';
import Equipment from './Equipment.mjs';
import Item from './Item.mjs';
import Energy from './Energy.mjs';
import WeaponItem from './WeaponItem.mjs';
import RechargeItem from './RechargeItem.mjs';
import ItemTemplate from './ItemTemplate.mjs';
import Inventory from './Inventory.mjs';
import ActionAttack from './ActionAttack.mjs';
import ActionEquip from './ActionEquip.mjs';
import ActionRecharge from './ActionRecharge.mjs';
import ActionPass from './ActionPass.mjs';
import ReactionShield from './ReactionShield.mjs';
import ReactionPass from './ReactionPass.mjs';
import Shuffle from './Shuffle.mjs';
import EnergyRechargerItem from './EnergyRechargerItem.mjs';

export default class SystemCharacter extends Character {
    constructor(name) {
        super(name);
    }

    inputAction() {
        super.inputAction();
        const left = this.equipment().get(Equipment.slots.left);
        const right = this.equipment().get(Equipment.slots.right);

        let energyOptions = [];
        if (this.hasEnergy(Energy.types.gamma, 1)) {
            energyOptions.push(Energy.types.gamma);
        }
        if (this.hasEnergy(Energy.types.chi, 1)) {
            energyOptions.push(Energy.types.chi);
        }
    
        const hasEnergy = (energyOptions.length > 0);

        let itemOptions = [];
        if (!left.isExhausted() && ( (hasEnergy && left instanceof WeaponItem) || (!hasEnergy && left instanceof RechargeItem)) ) {
            itemOptions.push(left);
        }
        if (!right.isExhausted() && ( (hasEnergy && right instanceof WeaponItem) || (!hasEnergy && right instanceof RechargeItem)) ) {
            itemOptions.push(right);
        }

        const hasItem = (itemOptions.length > 0);

        if (!hasEnergy) {
            // equip a recharger if one isn't available
            if (!(left instanceof RechargeItem || right instanceof RechargeItem)) {
                return new ActionEquip(this, Equipment.slots.left, this.inventory().findByClass(EnergyRechargerItem));
            } else {
                const rechargeItem = itemOptions.find(item => item instanceof EnergyRechargerItem);
                if (typeof rechargeItem !== 'undefined') {
                    return new ActionRecharge(this, rechargeItem, Shuffle.randomOption([Energy.types.gamma, Energy.types.chi]));
                } else {
                    return new ActionPass(this);
                }
            }
        } else if (!hasItem) {
            return new ActionPass(this);
        }

        const item = Shuffle.randomOption(itemOptions);

        if (item instanceof WeaponItem) {
            const energyType = Shuffle.randomOption(energyOptions);
            return new ActionAttack(this, this.getTarget(), item, energyType);
        } else if (item instanceof RechargeItem) {
            return new ActionRecharge(this, item, Shuffle.randomOption([Energy.types.gamma, Energy.types.chi]));
        } else {
            return new ActionPass(this);
        }
    }

    inputReaction(action) {
        super.inputReaction(action);

        let energyOptions = [];
        if (this.hasEnergy(Energy.types.gamma, 1)) {
            energyOptions.push(Energy.types.gamma);
        }
        if (this.hasEnergy(Energy.types.chi, 1)) {
            energyOptions.push(Energy.types.chi);
        }
        if (energyOptions.length === 0) {
            return new ReactionPass(action, this);
        }

        const energyType = Shuffle.randomOption(energyOptions);

        return new ReactionShield(action, this, energyType);
    }
}
