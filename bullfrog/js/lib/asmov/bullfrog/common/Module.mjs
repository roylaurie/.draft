'use strict';

import BullfrogConfig from "@asmov/bullfrog/common/Config";
import BullfrogCmdline from "@asmov/bullfrog/common/Cmdline";

export default class BullfrogModule {
    static namepath = 'asmov/bullfrog/common/Module';
    namepath = BullfrogModule.namepath;

    static MODULES_CFG_SCHEMA_NAME = 'modules';

    #cfg= new BullfrogConfig();
    #jsonPath = null;

    cmdline = null;

    constructor(packagePath) {
        this.#jsonPath = packagePath + '/json';
    }

    async init() {
        const schemaFilepath = this.#jsonPath + '/schema/package/modules.schema.json';
        const dataFilepath = this.#jsonPath + '/data/package/modules.json';

        await this.#cfg.loadSchema(BullfrogModule.MODULES_CFG_SCHEMA_NAME, schemaFilepath)
        const modulesCfg = await this.#cfg.loadData(BullfrogModule.MODULES_CFG_SCHEMA_NAME, dataFilepath);

        this.cmdline = new BullfrogCmdline(modulesCfg);
    }

    async start() {
        await this.init();

        const funcName = 'operation_' + this.cmdline.operation.replace('.', '_');
        this[funcName](this.cmdline.parameters, this.cmdline.options);
    }

    async stop() {

    }

    async shutdown() {

    }

    getConfig() {
        return this.#cfg;
    }

    config(schemaName, dataName = 'default') {
        return this.#cfg.config(schemaName, dataName);
    }
}