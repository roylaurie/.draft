'use strict';

import MetaLibrary from './util/Library.mjs';

export default class MetaTrait {
    static namepath = 'asmov/meta/js/Trait';
    static traitname = 'Trait';

    static schemas = {
        staticTraits: 'staticTraits',
        staticMethodTraits: 'staticMethodTraits',
        methodTraits: 'methodTraits',
        dataTraits: 'dataTraits'
    };

    /* Meta Convention: Static variables, typically immutable. e.g.; MyClass.namepath */
    static staticTraits = {
        namepath: 'namepath',
        traitname: 'traitname',
        staticTraits: 'staticTraits',
        staticMethodTraits: 'staticMethodTraits',
        methodTraits: 'methodTraits',
    };

    /* Meta Convention: Static method names. e.g.; MyClass.from() */
    static staticMethodTraits = {
        init: 'init'
    };

    /* Meta Convention: Method names that exist in a prototype */
    static methodTraits = {
        link: 'link',
        linked: 'linked',
        confirm: 'confirm',
        confirmLink: 'confirmLink',
        conform: 'conform',
        get: 'get'
    };

    /* Meta Convention: Variable names given to data as described by MetaModel */
    static dataTraits = {};

    /* Portable type names */
    static type = {
        bool: 'bool',
        byte: 'byte',
        uint16: 'uint16',
        uint32: 'uint32',
        uint64: 'uint64',
        uint128: 'uint128',
        int16: 'int16',
        int32: 'int32',
        int64: 'int64',
        int128: 'int128',
        ufloat16: 'ufloat16',
        ufloat32: 'ufloat32',
        ufloat64: 'ufloat64',
        ufloat128: 'ufloat128',
        float16: 'float16',
        float32: 'float32',
        float64: 'float64',
        float128: 'float128',
        utf8: 'utf8',
        utf16: 'utf16',
        utf32: 'utf32',
        map: 'map', // map<utf8, metatrait<MyClass>>
        array: 'array', // array<metatrait<MyClass>>
        metatrait: 'metatrait', // metatrait<Emitter>
        metatype: 'metatype',  // metatrait<MyClass> == metatrait<Emitter<MyClass>>
        metamodel: 'metamodel'  
    };

    static #librarySecret = Symbol();

    static init() {
        MetaLibrary.dot.reserve(MetaTrait.#librarySecret, MetaTrait);
        MetaTrait.link(MetaTrait);
    }

    constructor() {
        throw new Error('MetaTrait cannot be instantiated.');
    }

    confirm(metatrait) {
        for (let schema in MetaTrait.schemas) {
            if (typeof metatrait[schema] !== 'object') {
                throw new Error(`Trait schema '${schema}' missing in ${metatrait.namepath}`);
            }
        }

        MetaTrait.confirmTrait(metatrait, MetaTrait);
    }

    /** Registers a class with the MetaTrait library. Validates basic interace. Linked against a namespace defining MetaTraitPack. **/
    link(metatrait) {
        MetaTrait.confirm(metatrait);
        if (MetaTrait.linked(metatrait)) {
            throw new Error(`${metatrait.namepath} already linked'`);
        }

        MetaLibrary.dot.link(MetaTrait.#librarySecret, metatrait);
    }

    /** Determines whethere a class has been linked to the MetaTrait library or not. **/
    linked(metatrait) {
        return MetaLibrary.dot.has(MetaTrait, metatrait[MetaTrait.staticTraits.namepath]);
    }

    confirmLink(metatrait) {
        if (!MetaTrait.linked(metatrait)) {
            throw new Error(`${metatrait.namepath} is not link()'ed to MetaTrait`);
        }
    }

    // link MyClass to OtherClass by way of MyTrait
    // e.g., NamespaceTrait.linkTo(NamespaceClass, MyClass)
    //       ModelTrait.linkTo(ModelClass, FactoryClass)
    decorate(metatypeChild, metatrait, metatypeParent) {
        MetaLibrary.dot.decorate(MetaTrait.#librarySecret, metatype, metatrait);
    }

    conform(metatrait) {
        MetaTrait.confirm();
        MetaTrait.confirmLink(metatrait);
    }

    /** Retrieves the class linked for the given namespace **/
    get(namepath) {
        if (!MetaLibrary.dot.has(MetaTrait, namepath)) {
            throw new Error(`${namepath} namepath unknown`);
        }

        return MetaLibrary.dot.get(MetaTrait, namepath);
    }

    confirmTrait(metatype, metatrait) {
        for (let staticTrait in metatrait.staticTraits) {
            if (typeof metatype[staticTrait] === 'undefined') {
                throw new Error(`Static trait '${staticTrait}' missing in ${metatype.namepath}`);
            }
        }

        for (let staticMethodTrait in metatrait.staticMethodTraits) {
            if (typeof metatype[staticMethodTrait] !== 'function') {
                throw new Error(`Static method trait '${staticMethodTrait}' missing in ${metatype.namepath}`);
            }
        }

        for (let methodTrait in metatrait.methodTraits) {
            if (typeof metatype.prototype[methodTrait] !== 'function') {
                throw new Error(`Method trait '${methodTrait}' missing in ${metatype.namepath}`);
            }
        }
    }

    confirmTraitField(metatype, traitschema, trait) {
        switch(traitschema) {
        case MetaTrait.schemas.staticTraits:
            if (typeof metatype[trait] === 'undefined') {
                throw new Error(`${metatype[MetaTrait.staticTraits.namepath]} lacks a static ${trait}() variable`);
            }
            break;
        case MetaTrait.schemas.staticMethodTraits:
            if (typeof metatype[trait] !== 'function') {
                throw new Error(`${metatype[MetaTrait.staticTraits.namepath]} lacks a static ${trait}() function`);
            }
            break;
        case MetaTrait.schemas.methodTraits:
            if (typeof metatype.prototype === 'undefined' || typeof metatype.prototype[trait] !== 'function') {
                throw new Error(`${metatype[MetaTrait.staticTraits.namepath]} lacks a ${trait}() method`);
            }
            break;
        default:
            throw new Error(`Invalid traitschema '${traitschema}' for operation`);
        }
    }
}
