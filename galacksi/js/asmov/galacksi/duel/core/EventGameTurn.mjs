'use strict';

import EventGame from './EventGame.mjs';
import EventGameState from './EventGameState.mjs';
import Game from './Game.mjs';
import Round from './Round.mjs';
import Turn from './Turn.mjs';

export default class EventGameTurn extends EventGameState {
    static namepath = 'asmov/galacksi/EventGameTurn';

    static dataKeys = {
        turn: 'turn'
    };

    #turn = null;

    constructor(game, round, turn, timestamp) {
        super(game, round, timestamp);
        this.#turn = turn;
    }

    getTurn() {
        return this.#turn;
    }

    data() {
        return super.data().concat({
            EventGameTurn.dataKeys.turn: this.#turn.data()
        });
    };
}



