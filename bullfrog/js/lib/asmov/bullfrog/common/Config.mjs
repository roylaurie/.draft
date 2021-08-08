'use strict';

import BullfrogCommon from '@asmov/bullfrog/common/lib/Common';
import Ajv from "ajv";

export default class BullfrogConfig {
    static namepath = 'asmov/bullfrog/common/lib/Config';
    namepath = BullfrogConfig.namepath;

    #ajv = new Ajv();
    #data = {};
    #fileSystem = null;

    constructor(fileSystem) {
        this.#fileSystem = fileSystem;
    }

    async loadSchema(schemaName, filepath) {
        //TODO: make this platform-independent
        const jsonSchema = JSON.parse((await this.#fileSystem.read(filepath)).toString());
        this.#ajv.addSchema(jsonSchema, schemaName);
        this.#data[schemaName] = {};
    }

    async readData(schemaName, filepath) {
        //TODO: platform-indie
        const jsonData = JSON.parse((await this.#fileReader.read(filepath)).toString());
        const validateFunc = this.#ajv.getSchema(schemaName);
        if (!validateFunc(jsonData)) {
            throw new Error('Unable to parse invalid JSON.');
        }

        return jsonData;
    }

    configure(schemaName, data, dataName = 'default') {
        this.#data[schemaName][dataName] = data;
        BullfrogCommon.deepfreeze(this.#data[schemaName][dataName]);
    }

    async loadData(schemaName, filepath, dataName = 'default') {
        const data = await this.readData(schemaName, filepath);
        this.configure(schemaName, data, dataName);
        return this.#data[schemaName][dataName];
    }

    config(schemaName, dataName = 'default') {
        if (typeof this.#data[schemaName] === 'undefined' || typeof this.#data[schemaName][dataName] === 'undefined' ) {
            throw new Error('Configuration not found.');
        }

        return this.#data[schemaName][dataName];
    }
}

