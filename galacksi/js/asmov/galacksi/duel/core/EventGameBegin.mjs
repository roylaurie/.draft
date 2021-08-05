'use strict';

import EventGame from './EventGame.mjs';
import EventGameState from './EventGameState.mjs';
import Game from './Game.mjs';

/** @final **/
export default class EventGameBegin extends EventGameState {
    static namepath = 'asmov/galacksi/EventGameBegin';

    static from(data, datasource) {
        return new EventGameBegin(datasource.get(Game, data[EventGame.dataKeys.game]), data[Event.dataKeys.timestamp], data[ModelMeta.dataKeys.id]);
    }

    constructor(game, timestamp) {
        super(game, timestamp);
    }

    data() {
        return {
            ...super.data(),
            EventGame.dataKeys.game: this.game().data()
        };
    }
}
