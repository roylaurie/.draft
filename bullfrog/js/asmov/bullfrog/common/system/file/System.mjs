'use strict';

import fs from "fs/promises";
import fs from 'fs/promises';
import path from 'path';
import url from 'url';


export default class FileSystem {
    #basepath = null;

    static async urlToPath(url) {
        return path.dirname(url.fileURLToPath(import.meta.url);
    }

    constructor(basepath) {
        this.#basepath = basepath;
    }

    async urlToPath(url) {
        return FileSystem.urlToPath(url);
    }

    async realpath(path) {
        return fs.realpath(path);
    }

    path(path) {
        return this.#basepath + '/' + path;
    }

    async read(filepath) {
        return fs.readFile(filepath);
    }
}