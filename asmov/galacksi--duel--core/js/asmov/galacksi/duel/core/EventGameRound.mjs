'use strict';

import EventGameState from './EventGameState.mjs';
import Game from './Game.mjs';
import Round from './Round.mjs';

/** @abstract **/
export default class EventGameRound extends EventGameState {
    static namepath = 'asmov/galacksi/EventGameRound';

    static dataKeys = {
        round: 'round'
    };

    #round = null;

    constructor(game, round, timestamp, id) {
        super(game, timestamp, id);
        this.#round = round;
    }

    identify() {
        return MetaModel.identify(this, [ this.getTimestamp(), ModelMeta.identity(this.getGame()), ModelMeta.identity(this.#round) ]);
    }

    getRound() {
        return this.#round;
    }

    data() {
        return {
            ...super.data(),
            EventGameRound.dataKeys.round: MetaModel.identity(this.#round.data())
        };
    };
}


