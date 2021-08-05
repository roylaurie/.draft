'use strict';

import Reaction from './Reaction.mjs';
import Character from './Character.mjs';
import ActionAttack from './ActionAttack.mjs';
import WeaponItem from './WeaponItem.mjs';

export default class ReactionPass extends Reaction {
    constructor(action, reactor) {
        super(action, reactor);
    }

    react() {
        super.react();
        const action = this.getAction();
        const reactor = this.getReactor();

        if (action instanceof ActionAttack) {
            reactor.damage(1, action.getWeaponItem(), action.getActor());

            if (reactor.unconscious()) {
                console.log(reactor.getName(), 'passed and was knocked unconscious');
            } else {
                console.log(reactor.getName(), 'passed and was hit');
            }
        }
    }
}


