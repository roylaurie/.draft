'use strict';

import Meta from './Meta.mjs';
import ModelMeta from './ModelMeta.mjs';
import API from './API.mjs';
import EventGame from './EventGame.mjs';
import EventGameState from './EventGameState.mjs';
import Game from './Game.mjs';
import Round from './Round.mjs';
import Turn from './Turn.mjs';

export default class EventGameTurnStart extends EventGameTurnStart {
    static namepath = 'asmov/galacksi/EventGameTurnStart';

    static from(data, datasource) {
        return new EventGameTurnStart(
            datasource.get(Game, data[EventGame.dataKeys.game]),
            datasource.get(Round, data[EventGameRound.dataKeys.round],
            datasource.get(Turn, data[EventGameTurn.dataKeys.turn],
            data[Event.dataKeys.timestamp] );
    }

    constructor(game, round, turn, timestamp) {
        super(game, round, timestamp);
    }
}

Meta.dot.link(EventGameTurnStart);
ModelMeta.dot.registerClass(EventGameTurnStart);
