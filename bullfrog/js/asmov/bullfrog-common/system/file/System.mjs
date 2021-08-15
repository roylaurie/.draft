'use strict';

import fs from 'fs/promises';
import url from 'url';
import Common from '@asmov/bullfrog-common/lib/Common';


export default class FileSystem {
    static namepath = 'asmov/bullfrog-common/system/file/System';
    namepath = FileSystem.namepath;

    basepath = null;

    constructor(basepath) {
        this.basepath = basepath;
        Common.deepfreeze(this);
    }

    async metaPath(importMeta) {
        return url.fileURLToPath(import.meta);
    }

    async dirname(path) {
        return fs.dirname(path);
    }

    async realpath(path) {
        return fs.realpath(path);
    }

    path(path) {
        return this.basepath + '/' + path;
    }

    async read(filepath) {
        return fs.readFile(filepath);
    }
}

(()=>{
    Common.deepfreeze(FileSystem);
})();
