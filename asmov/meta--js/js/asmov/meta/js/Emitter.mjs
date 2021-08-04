'use strict';

import MetaTrait from './Trait.mjs';
import MetaType from './Type.mjs';
import MetaModel from './Model.mjs';

export default class MetaEmitter {
    static namepath = 'asmov/meta/js/Emitter';
    static traitname = 'Emitter';

    static staticTraits = {};
    static staticMethodTraits = {};
    static methodTraits = {
        listen: 'listen'
    };
    static dataTraits = {};

    #emittertypes = new Map();
    #MetaTrait = null;
    #MetaType = null;
    #MetaModel = null;

    constructor(metaTrait, metaType, metaModel) {
        this.#MetaTrait = metaTrait;
        this.#MetaType = metaType;
        this.#MetaModel = metaModel;
            
        this.#MetaTrait.link(MetaEmitter);
        this.#MetaType.link(MetaEmitter);
    }

    confirm(emittertype) {
        this.#MetaType.confirm(emittertype);
        this.#MetaModel.confirm(emittertype);
        this.#MetaTrait.confirmTrait(emittertype, MetaEmitter);
    }

    link(emittertype) {
        this.confirm(emittertype);
        this.#emittertypes.set(emittertype.namepath, emittertype);
    }


    linked(emittertype) {
        return this.#emittertypes.has(emittertype.namepath);
    }

    confirmLink(nametype) {
        if (!this.linked(nametype)) {
            throw new Error(`${nametype.namepath} is not link()'ed to MetaType`);
        }

        return;
    }

    conform(emittertype) {
        this.confirm(emittertype);
        this.confirmLink(emittertype);
        return;
    }

    get(namepath) {
        if (!this.#emittertypes.has(namepath)) {
            throw new Error(`${namepath} namepath unknown`);
        }

        return this.#emittertypes.get(namepath);
    }
}

