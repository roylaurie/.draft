'use strict';

import MetaTrait from '../../../../js/asmov/meta/js/Trait.mjs';
import MetaType from '.meta/asmov/meta/js/layer1/Type.mjs';
import MetaNamespace from './Namespace.mjs';
import MetaModel from './Model.mjs';
import MetaEmitter from './Emitter.mjs';

export default class Meta {
    static namepath = 'asmov/meta/js/Meta';

    static conformance = {
        freeze: 'freeze'
    };

    static Trait = MetaTrait;
    static Type = MetaType;
    static Namespace = MetaNamespace;
    //static Interface = MetaInterface;
    //static Source = Source;
    static Model = MetaModel;
    static Emitter = MetaEmitter;

    static meta = {
        namespace: null,
        implements: null,
        models: null,
        conforms: null
    };

    /** Post-processing for types that have been linked to a convention */
    static conform(metatype, conformance = {}) {
        Meta.Type.conform(metatype);

        if (Meta.Model.linked(metatype)) {
            Meta.Model.conform(metatype);
        }
        if (Meta.Emitter.linked(metatype)) {
            Meta.Emitter.conform(metatype);
        }

        if (typeof conformance[Meta.conformance.freeze] !== 'undefined' && conformance.freeze === false) {
            Object.seal(metatype);
        } else {
            Object.freeze(metatype); // default
        }

        return;
    }

    constructor() {
        if (Meta.Type.linked(Meta)) {
            throw new Error('Meta already initialized.');
        }

        Meta.Type.link(Meta);

        Meta.conform(MetaTrait);
        Meta.conform(MetaType);
        Meta.conform(MetaNamespace);
        Meta.conform(MetaModel);
        Meta.conform(MetaEmitter);
        Meta.conform(Meta);
    }

}
