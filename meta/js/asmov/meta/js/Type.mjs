'use strict';

import MetaTrait from './Trait.mjs';

export default class MetaType {
    static namepath = 'asmov/meta/js/Type';
    static traitname = 'Type';

    static staticTraits = {
        namepath: 'namepath',
    };

    static staticMethodTraits = {};

    static methodTraits = {};

    static dataTraits = {};


    #types = new Map();
    #MetaTrait = null;

    constructor(metaTrait) {
        metaTrait.entreat(MetaType);
        this.classify(MetaTrait);
        this.classify(MetaType);
    }

    confirm(metatype) {
        if (typeof metatype[MetaType.staticTraits.namepath] !== 'string') {
            throw new Error(`${metatype.namepath} missing namepath`);
        } 
    }

    /** Registers a class with the MetaType library. Validates basic interace. Linked against a namespace defining MetaTypePack. **/
    link(metatype) {
        this.confirm(metatype);
        if (this.linked(metatype)) {
            throw new Error(`${metatype.namepath} already linked'`);
        }

        this.#types.set(metatype.namepath, metatype);
    }

    /** Determines whethere a class has been linked to the MetaType library or not. **/
    linked(metatype) {
        return this.#types.has(metatype[MetaType.staticTraits.namepath]);
    }

    confirmLink(metatype) {
        if (!this.linked(metatype)) {
            throw new Error(`${metatype.namepath} is not link()'ed to MetaType`);
        }

        return;
    }

    conform(metatype) {
        this.confirm(metatype);
        this.confirmLink(metatype);
        return;
    }

    get(namepath) {
        if (!this.#types.has(namepath)) {
            throw new Error(`${namepath} metatype unknown`);
        }

        return this.#types.get(namepath);
    }
}
