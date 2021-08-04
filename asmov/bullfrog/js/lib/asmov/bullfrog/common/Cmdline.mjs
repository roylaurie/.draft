'use strict';

import BullfrogCommon from '@asmov/bullfrog/common/Common';
import FrogOpts from '@asmov/bullfrog/common/FrogOpts';

export default class BullfrogCmdline {
    static namepath = 'asmov/bullfrog/common/Cmdline';
    namepath = BullfrogCmdline.namepath;

    static optionCfg() {
        return [
            { flag: 'a', name: 'app', type: 'string', default: 'bullfrog', valid: 'namespace' },
            { flag: 'c', name: 'config', type: 'string', default: 'default', valid: 'namespace' },
            { flag: 'j', name: 'json', type: 'boolean', default: false, alias: 'output', value: 'json' },
            { flag: 'o', name: 'output', type: 'string', default: 'term', valid: 'namespace' },
            { flag: 'r', name: 'remote', type: 'string', default: null },
            { flag: 'x', name: 'debug', type: 'boolean', default: false },
            { flag: 'X', name: 'debugall', type: 'boolean', default: false },
        ];
    }

    static validators() {
       return {
          'version.3': /^\d+\.\d+\.\d+$/
       };
    }

    static settings() {
        return {};
    }

    static parameterCfg(module, operation, modulesCfg) {
        const config = modulesCfg.package.namespaces[module].operations[operation].parameters;
        const paramCfg = [];

        for (const name in config) {
            const cfg = config[name];
            paramCfg.push({
                name: name,
                type: cfg.type,
                default: ( typeof cfg.default !== 'undefined' ? cfg.default : null ),
                required: ( cfg.required === true ),
                position: ( typeof cfg.position !== 'undefined' ? cfg.position : null ),
                valid: null, //TODO
                enum: ( typeof cfg.enum !== 'undefined' ? cfg.enum : null )
            });
        }

        return paramCfg;
    }

    options = null;
    module = null;
    operation = null;
    parameters = null;

    constructor(modulesCfg, _args=null) {
        const optionCfg = this.getOptionCfg();
        const validators = this.getValidators();
        const settings = this.getSettings();
        const args = (_args === null ? process.argv.slice(2) : _args );
        const self = this;

        const frogopts = new FrogOpts(optionCfg, validators, settings).process(args, (module, operation) => {
            return self.getParameterCfg(module, operation, modulesCfg);
        });

        this.options = frogopts.options;
        this.module = frogopts.module;
        this.operation = frogopts.operation;
        this.parameters = frogopts.parameters;

        BullfrogCommon.deepfreeze(this);
    }

    getOptionCfg() {
        return BullfrogCmdline.optionCfg();
    }

    getValidators() {
        return BullfrogCmdline.validators();
    }

    getSettings() {
        return BullfrogCmdline.settings();
    }

    getParameterCfg(module, operation, modulesCfg) {
        return BullfrogCmdline.parameterCfg(module, operation, modulesCfg);
    }
}

BullfrogCommon.deepfreeze(BullfrogCmdline);