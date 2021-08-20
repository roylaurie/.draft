'use strict';

import BullfrogConfig from '@asmov/bullfrog.common/lib/Config';
import BullfrogCmdline from '@asmov/bullfrog.common/lib/Cmdline';
import Common from '@asmov/bullfrog.common/lib/Common';
import PackageStructure from '@asmov/bullfrog.common/lib/package/Structure';

export default class BullfrogModule {
    static namepath = 'asmov/bullfrog/common/lib/Module';
    namepath = BullfrogModule.namepath;

    static MODULES_CFG_SCHEMA_NAME = 'modules';

    fs = { package: null, module: null, operation: null };
    cmdline = null;

    #config = null;

    constructor(config, packagePath) {
        this.#config = config;
        this.fs.package = PackageStructure.build(PackageStructure.package, packagePath, this.fs);
    }

    async init() {
        const schemaFilepath = this.fs.package.json.schema.package.config.filepath;
        const dataFilepath = this.fs.package.json.data.package.config.filepath;

        await this.#cfg.loadSchema(BullfrogModule.MODULES_CFG_SCHEMA_NAME, schemaFilepath);
        const modulesCfg = await this.#cfg.loadData(BullfrogModule.MODULES_CFG_SCHEMA_NAME, dataFilepath);

        this.cmdline = new BullfrogCmdline(modulesCfg);

        const lang = modulesCfg.package.namespaces[this.cmdline.module].operations[this.cmdline.operation].lang;
        this.fs.module = PackageStructure.build(PackageStructure.module, this.fs.package.path, this.fs, this.cmdline.module, lang);
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