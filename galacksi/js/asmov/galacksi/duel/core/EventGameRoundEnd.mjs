'use strict';

import EventGame from './EventGame.mjs';
import EventGameRound from './EventGameRound.mjs';
import Game from './Game.mjs';
import Round from './Round.mjs';

export default class EventGameRoundEnd extends EventGameRound {
    static namepath = 'asmov/galacksi/EventGameRoundEnd';

    static from(data, datasource) {
        return new EventGameRoundEnd(datasource.get(Game, data[EventGame.dataKeys.game]),
                datasource.get(Round, data[EventGameRound.dataKeys.round], data[Event.dataKeys.timestamp]);
    }


    constructor(game, round, timestamp) {
        super(game, round, timestamp);
    }
}


