'use stict';

import Character from './Character.mjs';

export default class PlayerCharacter extends Character {
    constructor(name) {
        super(name);
    }

    inputAction() {
        super.inputAction();
    }

    inputReaction(action) {
        super.inputReaction(action);
    }
}
