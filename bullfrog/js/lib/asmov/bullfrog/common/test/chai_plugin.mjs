'use strict';

import BullfrogCommon from '@asmov/bullfrog/common/Common';

export default (chai, utils) => {
    utils.addProperty(chai.Assertion.prototype, 'frog_namespace', function() {
        this.assert(
            BullfrogCommon.validNamespace(this._obj),
            "expected #{this} to be a valid bullfrog namespace",
            "expected #{this} to not be a valid bullfrog namespace"
        );
    });

    utils.addProperty(chai.Assertion.prototype, 'frog_class', function() {
        this.assert(
            BullfrogCommon.validNamepath(this._obj.namepath),
            "expected #{this} class to have a valid bullfrog namepath field",
            "expected #{this} class to not have a valid bullfrog namepath field"
        );

        new chai.Assertion(this._obj).to.be.frozen;
    });

    utils.addProperty(chai.Assertion.prototype, 'frog_class_object', function() {
        new chai.Assertion(this._obj.constructor).to.be.a.frog_class;
        this.assert(
            BullfrogCommon.validNamepath(this._obj.namepath),
            "expected #{this} to have a valid bullfrog namepath field",
            "expected #{this} to not be a valid bullfrog namepath field"
        );
        this.assert(
            this._obj.namepath === this._obj.constructor?.namepath,
            "expected #{this} to have the same namepath field as its class",
            "expected #{this} to not have the same namepath field as its class"
        );
    });
}