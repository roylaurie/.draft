'use strict';

import Reaction from './Reaction.mjs';
import Energy from './Energy.mjs';
import Action from './Action.mjs';
import Character from './Character.mjs';
import ActionAttack from './ActionAttack.mjs';

export default class ReactionShield extends Reaction {
    #shieldEnergyType = null;

    constructor(action, reactor, shieldEnergyType) {
        super(action, reactor);
        this.#shieldEnergyType = shieldEnergyType;
    }

    react() {
        super.react();
        const action = this.getAction();
        const reactor = this.getReactor();

        if (action instanceof ActionAttack) {
            if (this.#shieldEnergyType === action.getEnergyType()) {
                try {
                    reactor.consumeEnergy(this.#shieldEnergyType, 1);
                    console.log(reactor.getName(), 'blocked the shot with a', Energy.names[this.#shieldEnergyType], 'shield');
                } catch(e) {
                    reactor.damage(1, action.getWeaponItem(), action.getActor());
                    console.log(reactor.getName(), 'was hit, failing to raise a', Energy.names[this.#shieldEnergyType], 'shield');
                }
            } else {
                reactor.damage(1, action.getWeaponItem(), action.getActor());
                console.log(reactor.getName(), 'was hit, penetrating a', Energy.names[this.#shieldEnergyType], 'shield');
            }

            if (reactor.unconscious()) {
                console.log(reactor.getName(), 'was knocked unconscious');
            }
        }
    }
}


