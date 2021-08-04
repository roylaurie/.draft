'use strict';

export default class Round {
    #game = null;

    constructor(game) {
        this.#game = game;
    }

    start() {
        const characters = this.#game.getCharacters();
        for (let i = 0; i < characters.length; ++i) {
            const character = characters[i];
            if (!character.conscious()) {
                continue;
            }

            const action1 = character.inputAction();

            action1.act();

            if (this.#game.numConscious() == 1) {
                this.end();
                return;
            }

            const action2 = character.inputAction();
            action2.act();

            if (this.#game.numConscious() == 1) {
                this.end();
                return;
            }
        }

        this.end();
        return;
    }

    end() {
    }
}


