'use strict';

import Meta from '.meta/asmov/meta/js/Meta.mjs';
import GalacksiDuelCoreCodebase from '.meta/asmov/galacksi/duel/core/MetaCodebase.mjs';

/** @abstract **/
export default class Event {
    static namepath = 'asmov/galacksi/duel/core/Event';
    static meta = [
        Meta.attrib.Abstract,
        Meta.attrib.Model
    ];

    static dataKeys = {
        timestamp: 'timestamp'
    };

    #timestamp = null;
    #id = null;

    constructor(timestamp, id) {
        this.#timestamp = timestamp || Date.UTC();
        this.#id = id || this.identify();
    }

    id() {
        return this.#id;
    }

    identify() {
        return ModelMeta.id32(this, [ this.#timestamp ]);
    }

    getTimestamp() {
        return this.#timestamp;
    }

    data() {
        return {
            ModelMeta.dataKeys.namepath: this.constructor.namepath,
            Event.dataKeys.timestamp: this.#timestamp
        };
    }
}

Meta.link(GalacksiPack, Event);
