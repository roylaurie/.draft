'use strict';

import Meta from '../../asmov/meta/js/Meta.mjs';
import MetaModel from '../../asmov/meta/js/Model.mjs';
import Event from './Event.mjs';
import Game from './Game.mjs';


/** @abstract **/
export default class EventGame extends Event {
    static namepath = 'asmov/galacksi/EventGame';

    static dataKeys = {
        game: 'game'
    };

    #game = null;

    constructor(game, timestamp, id) {
        super(timestamp, id);
        this.#game = game;
    }

    identify() {
        return MetaModel.identify(this, [ this.getTimestamp(), MetaModel.identity(this.#game) ]);
    }

    getGame() {
        return this.#game;
    }

    data() {
        return {
            ...super.data(),
            EventGame.dataKeys.game: MetaModel.identity(this.#game)
        });
    }
}
