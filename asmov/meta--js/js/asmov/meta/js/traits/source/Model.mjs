'use strict';

import Meta from '../Meta.mjs';
import MetaDatasource from '../Datasource.mjs';
import MetaJSPack from '../MetaPack.mjs';

export default class ModelSource {
    static namepath = 'asmov/meta/js/model/Datasource';

    #contextpath = null;
    #data = new Map();

    constructor(contextpath) {
        this.#contextpath = contextpath;
    }

    context() {
        return this.#contextpath;
    }

    store(modelobject) {
        Meta.Model.conformsLink(modelobject.constructor);
        const namepath = modelobject.constructor.namepath;

        if (!this.#data.has(namepath)) {
            this.#data.set(namepath) = new Map();
        }

        this.#data.get(namepath).set(modelobject.id(), modelobject);
    }

    retrieve(identity) {
        Meta.Model.conformsDataIdentity(identity);
        const store = this.#data.get(identity.namepath);
        if (typeof table === 'undefined') {
                return null;
        }
        
        return store.get(identity.id) || null;
    }

    from(data) {
        const identity = Meta.Model.dataIdentity(data);
        const object = this.retrieve(identity);
        if (object !== null) {
            return object;
        } else {
            const newObject = Meta.Model.from(data, this);
            this.store(newObject);
            return newObject;
        }
    }
}

Meta.Type.link(ModelSource);
Meta.Namespace.use(ModelSource, MetaJSPack);
Meta.conform(ModelSource);
