'use strict';

export default class BullfrogCommon {
    static namepath = 'asmov/bullfrog/Common';

    static #namespacePattern = /^(?![\d.]+$)(?:[a-z0-9]+(?:\.[a-z0-9]+)*)+$/

    static validNamespace(str) {
        return ( BullfrogCommon.#namespacePattern.test(str) );
    }

    static deepfreeze(object) {
        const propNames = Object.getOwnPropertyNames(object);

        for (const name of propNames) {
            const value = object[name];

            if (value && typeof value === "object") {
                BullfrogCommon.deepfreeze(value);
            }
        }

        return Object.freeze(object);
    }

    constructor() { throw new Error('BullfrogCommon cannot be instantiated.'); }
}

BullfrogCommon.deepfreeze(BullfrogCommon);