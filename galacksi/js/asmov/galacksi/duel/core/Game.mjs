'use strict';

import Character from './Character.mjs';
import SystemCharacter from './SystemCharacter.mjs';
import Round from './Round.mjs';
import Energy from './Energy.mjs';

export default class Game {
    #characters = [];
    #rounds = [];
    #currentRound = null;

    constructor() {
        const p1 = new SystemCharacter('Player 1');
        const p2 = new SystemCharacter('Player 2');

        this.#characters.push(p1);
        this.#characters.push(p2);

        p1.setTarget(p2);
        p2.setTarget(p1);
    }

    getCharacters() {
        return this.#characters;
    }

    getConsciousCharacters() {
        const consciousCharacters = [];
        for (let i = 0; i < this.#characters.length; ++i) {
            if (this.#characters[i].conscious()) {
                consciousCharacters.push(this.#characters[i]);
            }
        }

        return consciousCharacters;
    }

    numConscious() {
        let conscious = 0;
        for (let i = 0; i < this.#characters.length; ++i) {
            if (this.#characters[i].conscious()) {
                ++conscious; 
            }
        }

        return conscious;
    }

    start() {
        let roundNum = 0;
        while (this.numConscious() > 1) {
            this.#currentRound = new Round(this);
            this.#rounds.push(this.#currentRound);

            console.log('Round #' + ++roundNum + ' starting ...');
            for (let i = 0; i < this.#characters.length; ++i) {
                const c = this.#characters[i];
                c.initRound(this.#currentRound);

                const o = { omega: c.getOmegaPoints(), energy: { gamma: c.getEnergy(Energy.types.gamma), chi: c.getEnergy(Energy.types.chi) } };
                console.log(c.getName(), o);
            }

            this.#currentRound.start();
        }

        console.log(this.getConsciousCharacters()[0].getName(), 'wins!');
        console.log('Game over');
    }
};
