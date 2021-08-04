#!/usr/bin/env node
'use strict';

import BullfrogModule from '@asmov/bullfrog/common/Module';
import BullfrogCommon from '@asmov/bullfrog/common/Common';

class VersionModule extends BullfrogModule {
    static namepath = 'asmov/bullfrog/module/common/SysInfo';
    namepath = VersionModule.namepath;

    static #versionRegex = /^\d+\.\d+\.\d+/

    constructor() {
        super('.');
    }

    operation_js(parameters, options) {
        console.log(this);

        const nodejsVersion = process.versions['node'].match(VersionModule.#versionRegex)[0];
        const v8Version = process.versions['v8'].match(VersionModule.#versionRegex)[0];

        console.log('NodeJS Version: ', nodejsVersion);
        console.log('V8 Version: ', v8Version);
    }
}

BullfrogCommon.deepfreeze(VersionModule);

new VersionModule().start();