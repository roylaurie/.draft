'use strict';

export default class BullfrogCommon {
    static namepath = 'asmov/bullfrog/Common';

    static #namespacePattern = /^(?![\d.]+$)(?:[a-z0-9]+(?:\.[a-z0-9]+)*)+$/
    static #namepathPattern = /^(?![\d.\/]+$)(?:[a-z0-9]+(?:\/[a-z0-9]+)*)+\/(?![\d.\/]+$)(?:[a-zA-Z0-9_]+(?:([._]|::)[a-zA-Z0-9_]+)*)+$/

    static validNamespace(str) {
        return ( typeof str === 'string' && this.#namespacePattern.test(str) );
    }

    static validNamepath(str) {
        return ( typeof str === 'string' && this.#namepathPattern.test(str) );
    }

    static deepfreeze(object) {
        const propNames = Object.getOwnPropertyNames(object);

        for (const name of propNames) {
            const value = object[name];

            if (value && typeof value === 'object') {
                this.deepfreeze(value);
            }
        }

        return Object.freeze(object);
    }

    static path(packagePath, suffix) {
        return packagePath + suffix;
    }



    constructor() { throw new Error('BullfrogCommon cannot be instantiated.'); }
}

BullfrogCommon.deepfreeze(BullfrogCommon);