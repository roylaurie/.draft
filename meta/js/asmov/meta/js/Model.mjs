'use strict';

import MetaTrait from './Trait.mjs';
import MetaType from './Type.mjs';

export default class MetaModel {
    static namepath = 'asmov/meta/js/Model';
    static traitname = 'Model';

    static staticTraits = {}; // best typo ever: staticTreats

    static staticMethodTraits = {
        from: 'from'
    };

    static methodTraits = {
        data: 'data',
        id: 'id'
    }

    static dataTraits = {
        namepath: 'namepath',
        id: 'id',
    }

    #modeltypes = new Map();
    #MetaTrait = null;
    #MetaType = null;

    constructor(metaTrait, metaType) {
        this.#MetaTrait = metaTrait;
        this.#MetaType = metaType;

        metaTrait.link(MetaModel);
        metaType.link(MetaModel);
    }

    confirm(modeltype) {
        this.#MetaTrait.confirm(modeltype);
        this.#MetaTrait.confirmTrait(modeltype, MetaModel);
        return;
    }

   link(modeltype) {
        this.confirm(modeltype);
        this.#MetaTrait.confirmLink(modeltype);
        this.#modeltypes.set(modeltype.namepath, modeltype.constructor.from);
    }

    linked(modeltype) {
        return this.#modeltypes.has([modeltype[MetaTrait.staticTraits.namepath]]);
    }

    define(modeltype) { this.link(modeltype); }

    defined(modeltype) { return this.linked(modeltype); }

    confirmLink(modeltype) {
        if (!this.linked(modeltype)) {
            throw new Error(`${modeltype.namepath} is not link()'ed to MetaType`);
        }

        return;
    }

    conform(modeltype) {
        this.confirm();
        this.confirmLink(modeltype);
        return;
    }

    get(namepath) {
        if (!this.#modeltypes.has(namepath)) {
            throw new Error(`${namepath} namepath unknown`);
        }

        return this.#modeltypes.get(namepath);
    }

    confirmDataTrait(data, trait) {
        if (typeof data[trait] === 'undefined') {
            throw new Error(`Data '${data[MetaModel.dataTraits.namepath]}' + ' lacks a '${trait}' key`);
        }

        return;
    }

    confirmDataIdentity(data) {
        this.confirmDataTrait(data, MetaModel.dataTraits.namepath);
        this.confirmDataTrait(data, MetaModel.dataTraits.id);
    }

    from(data, datasource) {
        const namepath = data[MetaModel.dataTraits.namepath];
        const factoryMethod = this.#modeltypes.get(namepath);
        if (typeof factoryMethod === 'undefined') {
            throw new Error(`Data could not be factoried for namepath '${namepath}`);
        }

        return factoryMethod(data, datasource);
    }

    data(modeltype) {
        this.confirmLink(modeltype);
        return modeltype.data();
    }

    identity(modeltype) {
        this.confirmLink(modeltype);
        return Object.freeze({
            namepath: modeltype.namepath, 
            id: modeltype.id()
        });
    }

    dataIdentity(data) {
        this.confirmDataIdentity(data);
        return Object.freeze({
            namepath: data.namepath, 
            id: data.id
        });
    }
}


