#!/usr/bin/env node
'use strict';

import BullfrogCommon from '@asmov/bullfrog-common/lib/Common';
import BullfrogModule from '@asmov/bullfrog-common/lib/Module';
import BullfrogConfig from '@asmov/bullfrog-common/lib/Config';
import FileSystem from '@asmov/bullfrog-common/system/file/System';

class VersionModule extends BullfrogModule {
    static namepath = 'asmov/bullfrog/common/modules/system/lang/Version';
    namepath = VersionModule.namepath;

    static #versionRegex = /^\d+\.\d+\.\d+/

    constructor() {
        super(new FileSystem(FileSystem.urlToPath(import.meta.url)));
    }

    operation_js(parameters, options) {
        const nodejsVersion = process.versions['node'].match(VersionModule.#versionRegex)[0];
        const v8Version = process.versions['v8'].match(VersionModule.#versionRegex)[0];

        console.log('NodeJS Version: ', nodejsVersion);
        console.log('V8 Version: ', v8Version);
    }
}

(()=>{
    BullfrogCommon.deepfreeze(VersionModule);
    new VersionModule().start();
})();