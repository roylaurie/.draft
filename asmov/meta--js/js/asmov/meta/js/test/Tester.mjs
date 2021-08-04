'use strict';

import TestCase from './Test.mjs';

export default class Tester {
    static #testMethodPrefixRegex = /^test_/;

    testUnit(testpath) {
        const promise = import(testpath).then(module => {
            let currentPromise = Promise.resolve();  // the promise for this tests case
            const TestCase = module.default;
            const properties = Object.getOwnPropertyNames(TestCase.prototype);

            // instantiate the tests case and set it up
            const testCase = new TestCase();
            testCase.setup();

            // run any method that begins with 'test_'.
            for (let property of properties) {
                if (typeof TestCase.prototype[property] !== 'function' || !Tester.#testMethodPrefixRegex.test(property)) {
                    continue;
                }

                // run each tests method sequentially
                currentPromise = currentPromise.then(value => {
                    return self.testMethod(testObject, property);
                });
            }

            // teardown the tests case after all tests methods have completed.
            currentPromise.then(()=>{ testObject.teardown(); });

            return currentPromise;
        });
    }
}


test() {
    const self = this;
    let promise = Promise.resolve();

    for (let i = 0; i < this._testClassNamepaths.length; ++i) {
        const testClass = this._testClassNamepaths[i];
        promise = promise.then(value => {
            return self.testClass(testClass);
        });
    }

    return promise;
};

testClass(namepath) {
    const self = this;
    const filepath = '../../' + namepath + '.mjs';  //@todo use filesystem class

    this._log.info('Testing with ' + namepath + ' ...');

    const promise = import(filepath).then(module => {
        let currentPromise = Promise.resolve();  // the promise for this tests case
        const testClass = module.default;
        const properties = Object.getOwnPropertyNames(testClass.prototype);

        // instantiate the tests case and set it up
        const testObject = new testClass();
        testObject.setup();

        // run any method that begins with 'test_'.
        for (let i = 0; i < properties.length; ++i) {
            const property = properties[i];
            if (!/^test_/.test(property) || typeof testClass.prototype[property] !== 'function') {
                continue;
            }

            // run each tests method sequentially
            currentPromise = currentPromise.then(value => {
                return self.testMethod(testObject, property);
            });
        }

        // teardown the tests case after all tests methods have completed.
        currentPromise.then(()=>{ testObject.teardown(); });

        return currentPromise;
    });

    return promise;
};

testMethod(testObject, methodName) {
    const self = this;

    const promise = new Promise(function(resolve, reject) {
        self._log.info('Testing method ' + methodName + '()');

        try {
            const retval = testObject[methodName]();
            if (retval instanceof Promise) {  // then resolve the tests asynchronously
                retval.then(value => { resolve(); }).catch(e => { reject(e); });
            } else {
                resolve();
            }
        } catch (e) {
            reject(e);
        }
    });

    return promise;
};

/**
 * Fires an event to the tests case emitter.
 *
 * @param {String} subtopic
 * @param {obj} data
 */
_emit(subtopic, data) {
    this._emitter.emit(this._TOPIC + subtopic, data);
};

/**
 * Fires a tests case result event to the emitter. Conforms to xUnit XML result format.
 *
 * @param {String} methodName The tests method name.
 * @param {String} outcome 'Pass', 'Fail', 'Skip'
 * @param {number} time Elapsed time in milliseconds.
 */
_emitResult(methodName, outcome, time) {
    // @todo this._emitter.emit(this._RESULT_TOPIC, new Result(methodName, outcome, time).model());
    this._emitter.emit(this._RESULT_TOPIC, {
        name: methodName.substr(5), // remove the prefixed test_
        type: this.namepath,
        method: methodName,
        time: time,
        result: outcome
    });
};
}
