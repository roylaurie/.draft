#!/usr/bin/env node
'use strict';

import BullfrogModule from '@asmov/bullfrog/common/Module';
import BullfrogCommon from '@asmov/bullfrog/common/Common';
import fs from 'fs/promises';
import path from 'path';
import url from 'url';

const BASEPATH = await fs.realpath(path.dirname(url.fileURLToPath(import.meta.url)) + '/../../../..');

class VersionModule extends BullfrogModule {
    static namepath = 'module/asmov/bullfrog/common/lang/Version';
    namepath = VersionModule.namepath;

    static #versionRegex = /^\d+\.\d+\.\d+/

    constructor() {
        super(BASEPATH);
    }

    operation_js(parameters, options) {
        const nodejsVersion = process.versions['node'].match(VersionModule.#versionRegex)[0];
        const v8Version = process.versions['v8'].match(VersionModule.#versionRegex)[0];

        console.log('NodeJS Version: ', nodejsVersion);
        console.log('V8 Version: ', v8Version);
    }
}

BullfrogCommon.deepfreeze(VersionModule);

new VersionModule().start();