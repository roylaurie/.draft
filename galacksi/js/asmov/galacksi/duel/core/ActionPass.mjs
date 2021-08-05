'use strict';

import Action from './Action.mjs';
import Character from './Character.mjs';

export default class ActionPass extends Action {
    constructor(actor) {
        super(actor);
    }

    act() {
        super.act();
        console.log(this.getActor().getName(), 'passes');
    }
}


