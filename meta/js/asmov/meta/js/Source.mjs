'use strict';

import MetaTrait from './Trait.mjs';
import MetaType from './Type.mjs';

export default class MetaSource {
    static namepath = 'asmov/meta/js/Model';
    static traitname = 'Source';

    static staticTraits = {};
    static staticMethodTraits = {};
    static dataTraits = {};

    static methodTraits = {
        context: 'context',
        store: 'store',
        retrieve: 'retrieve',
        get: 'get'
    }

    static dot = new MetaSource();

    #sourcetypes = new Map();

    constructor() {
        if (typeof dot !== 'undefined') {
            throw new Error('MetaSource already initialized');
        }
    }

    confirm(sourcetype) {
        MetaType.dot.confirm(sourcetype);
        MetaTrait.dot.confirmTrait(sourcetype, MetaSource);
        return;
    }

    link(sourcetype) {
        this.confirm(sourcetype);
        MetaType.dot.confirmLink(sourcetype);
        this.#sourcetypes.set(sourcetype.namepath, sourcetype);
    }

    linked(sourcetype) {
        return this.#sourcetypes.has(sourcetype[MetaType.staticTraits.namepath]);
    }

    confirmLink(nametype) {
        if (!this.linked(nametype)) {
            throw new Error(`${nametype.namepath} is not link()'ed to MetaType`);
        }

        return;
    }

    conform(sourcetype) {
        this.confirm(sourcetype);
        this.confirmLink(sourcetype);
        return;
    }

    get(namepath) {
        if (!this.#sourcetypes.has(namepath)) {
            throw new Error(`${namepath} namepath unknown`);
        }

        return this.#sourcetypes.get(namepath);
    }
}

MetaTrait.dot.link(MetaSource);
MetaType.dot.link(MetaSource);

