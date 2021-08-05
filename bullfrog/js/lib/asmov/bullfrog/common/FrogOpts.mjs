'use strict';

import BullfrogCommon from '@asmov/bullfrog/common/Common';

export default class FrogOpts {
    static namepath = 'asmov/bullfrog/common/FrogOpts';
    namepath = FrogOpts.namepath;

    static inputClasses = {
        option: 'option',
        parameter: 'parameter'
    };

    static inputTypes = [ 'string', 'number', 'boolean' ];

    static #configInput(inputClass, cfg) {
        const config = {};

        if (!FrogOpts.inputClasses.hasOwnProperty(inputClass)) {
            throw new Error(`Invalid input type '${inputClass}'`);
        }

        const isOption = inputClass === FrogOpts.inputClasses.option;
        const isParameter = inputClass === FrogOpts.inputClasses.parameter;

        this.#configInputSetting(cfg, 'name', 'namespace', null, config, true);
        this.#configInputSetting(cfg, 'type', 'string', null, config, true, Object.keys(FrogOpts.inputClasses));
        this.#configInputSetting(cfg, 'flag', 'char', null, config, true, null, isOption);
        this.#configInputSetting(cfg, 'required', 'boolean', false, config);
        this.#configInputSetting(cfg, 'default', 'any', null, config);
        this.#configInputSetting(cfg, 'enum', 'array<string>', null, config);
        this.#configInputSetting(cfg, 'valid', 'string', null, config);
        this.#configInputSetting(cfg, 'position', 'integer', null, config, false, null, isParameter);
        //TODO further validation after all inputs have been configured
        this.#configInputSetting(cfg, 'alias', 'namespace', null, config, false, null, isOption);
        this.#configInputSetting(cfg, 'value', 'any', null, config, false, null, isOption);

        for (const key in cfg) {
           if (typeof config[key] === 'undefined') {
               throw new Error(`Illegal configuration setting for '${config.name}' : ${key}`);
           }
        }

        return config;
    }

    static #configInputSetting(inputCfg, property, typeofExpected, defaultValue, outputCfg,
                               required=false, restrictions=null, allowed=true) {
        if (typeof inputCfg[property] === 'undefined') {
            if (required && allowed) {
                throw new Error(`Missing required input config setting '${property}'`);
            }

            outputCfg[property] = defaultValue;
            return;
        } else if (!allowed) {
            throw new Error('Illegal config setting for input class');
        }

        const value = inputCfg[property];
        let error = false;

        switch(typeofExpected) {
            case 'any':
                break;
            case 'array<string>':
                error = !Array.isArray(value);
                for (let i = 0, n = value.length; !error && i < n ; ++i) {
                    error = ( typeof value[i] !== 'string' );
                }
                break;
            case 'integer':
                error = ( typeof value !== 'number' || !Number.isInteger(value) );
                break;
            case 'char':
                error = ( typeof value !== 'string' || value.length !== 1)
                break;
            case 'namespace':
                error = ( typeof value !== 'string' || !BullfrogCommon.validNamespace(value) );
                break;
            default:
                error = (typeof value !== typeofExpected);
        }

        if (!error && restrictions !== null) {
            error = ( restrictions.includes(value) )
        }

        if (error) {
            throw new Error(`Illegal value in '${property}' setting for input config '${inputCfg.name}'`);
        }

        outputCfg[property] = value;
    }

    #optionCfg = {};
    #flagNames = {};
    #settings = { cache: true };
    #paramCfgCache = {};
    #validators = {};
    #cursor = 0;

    constructor(optionCfg, validators={}, settings={}) {
        const optionClass = FrogOpts.inputClasses.option;

        for (const cfg of optionCfg) {
            const config = FrogOpts.#configInput(optionClass, cfg);
            this.#optionCfg[config.name] = config;
            this.#flagNames[config.flag] = config.name;
        }

        for (const name in validators) {
            const validator = validators[name];
            if (!(validator instanceof Function) && !(validator instanceof RegExp)) {
                throw new Error(`Invalid validator '${name}'`)
            }

            this.#validators[name] = validator;
        }

        if (typeof settings.cache !== 'undefined') {
            this.#settings.cache = !!settings.cache
        }
    }

    parse(args) {
        this.#cursor = 0;

        let options = this.#parseOptions(args);
        const [module, operation] = this.#parseCommand(args);
        let parameters = this.#parseParameters(args);

        return { options: options, module: module, operation: operation, parameters: parameters };
    }

    process(args, parameterCfg = null) {
        const parsed = this.parse(args);
        const options = this.#processOptions(parsed.options);
        const cacheKey = parsed.module + '.' + parsed.operation;
        let parameters = parsed.parameters;
        let parameterConfig = null;

        if (this.#settings.cache && typeof this.#paramCfgCache[cacheKey] !== 'undefined') {
            parameterConfig = this.#paramCfgCache[cacheKey];
        } else if (parameterCfg !== null) {
            if (typeof parameterCfg === 'function') {
                parameterCfg = parameterCfg(parsed.module, parsed.operation)
            }

            const parameterClass = FrogOpts.inputClasses.parameter;
            parameterConfig = {};
            for (const cfg of parameterCfg) {
                const config = FrogOpts.#configInput(parameterClass, cfg);
                parameterConfig[config.name] = config;
            }

            if (this.#settings.cache) {
                this.#paramCfgCache[cacheKey] = parameterConfig;
            }
        }

        if (parameterConfig !== null) {
            parameters = this.#processParameters(parsed.parameters, parameterConfig);
        }

        return { options: options, module: parsed.module, operation: parsed.operation, parameters: parameters };
    }

    #parseOptions(args) {
        const numArgs = args.length;
        const options = {};
        let valOpt = null;

        for ( ; this.#cursor < numArgs ; ++this.#cursor) {
            const arg = args[this.#cursor];

            if (valOpt !== null) {
                options[this.#flagNames[valOpt]] = arg;
                valOpt = null;
            } else {
                if (arg[0] === '-') {
                    for (let i = 1, n = arg.length; i < n; ++i) {
                        const o = arg[i];

                        if (typeof this.#flagNames[o] === 'undefined') {
                            throw new Error(`Unknown option '${o}'`);
                        }

                        const name = this.#flagNames[o];
                        if (this.#optionCfg[name].type === 'boolean') {
                            options[name] = true;
                        } else if (i + 1 < arg.length) {
                            throw new Error(`Missing value for option '${o}'`);
                        } else {
                            valOpt = o;
                        }
                    }
                } else {
                    break;
                }
            }
        }

        return options;
    }

    #parseCommand(args) {
        const numArgs = args.length;

        if (this.#cursor >= numArgs) {
            throw new Error('Missing module')
        } else if (!BullfrogCommon.validNamespace(args[this.#cursor])) {
            throw new Error(`Illegal module name '${args[this.#cursor]}'`);
        }

        const module = args[this.#cursor++];
        let operation = 'default';

        if (this.#cursor < numArgs && args[this.#cursor][0] !== '-') {
            if (!BullfrogCommon.validNamespace(args[this.#cursor])) {
                throw new Error(`Illegal operation name '${args[this.#cursor]}'`);
            }

            operation = args[this.#cursor++];
        }

        return [module, operation];
    }

    #parseParameters(args) {
        const numArgs = args.length;
        const params = {};
        const cursorFirst = this.#cursor;
        let valParam = null;
        let posCapable = true;

        for ( ; this.#cursor < numArgs ; ++this.#cursor) {
            const arg = args[this.#cursor];

            if (valParam !== null) {
                params[valParam] = arg;
                valParam = null;
            } else if (arg[0] === '-') {
                if (arg[1] !== '-') {
                    throw new Error(`Illegal placement of option '${arg}' in parameter string`);
                }

                valParam = arg.slice(2);
                posCapable = false;
                if (!BullfrogCommon.validNamespace(valParam)) {
                    throw new Error(`Ilegal parameter name '${valParam}'`);
                }
            } else if (posCapable) {
                params[this.#cursor - cursorFirst + 1] = arg;
            } else {
                throw new Error(`Illegal placement of positional parameter: ${arg}`);
            }
        }

        return params;
    }

    #processOptions(_options) {
        const options = {};
        const optionInput = FrogOpts.inputClasses.option;

        for (const name in this.#optionCfg) {
            const optCfg = this.#optionCfg[name];
            const value = _options[name];
            const result = this.#processInput(optionInput, optCfg, value);

            if (result !== null) {
                options[name] = result;
            }
        }

        return options;
    }

    #processParameters(_parameters, parameterCfg) {
        const parameters = {};

        for (const _name in _parameters) {
            let name = null;

            const pos = Number(_name);
            if (Number.isInteger(pos)) {
                for (const configName in parameterCfg) {
                    if (parameterCfg[configName].position === pos) {
                        name = configName;
                        break;
                    }
                }

                if (name === null) {
                    throw new Error(`Unexpected positional parameter '${_parameters[name]}'`);
                }

                _parameters[name] = _parameters[_name];
                delete _parameters[_name];
            } else if (typeof parameterCfg[_name] === 'undefined') {
                throw new Error(`Undefined parameter '${_name}'`);
            }
        }

        for (const name in parameterCfg) {
            const cfg = parameterCfg[name];
            const value = this.#processInput(FrogOpts.inputClasses.parameter, cfg, _parameters[name]);
            if (value !== null) {
                parameters[name] = value;
            }
        }

        return parameters;
    }

    #processInput(inputClass, cfg, value) {
        const name = cfg.name;
        let result = null;

        if (cfg.required && typeof value === 'undefined') {
            throw new Error(`Missing required ${cfg.position !== null ? 'positional ' : ''}parameter '${name}'`);
        }

        if (typeof value !== 'undefined') {
            if (cfg.enum !== null && !cfg.enum.includes(value)) {
                throw new Error(`Invalid option for enum parameter '${name}'`);
            }

            result = this.#processInputValue(cfg, value);
        } else if (cfg.default !== null) {
            result = this.#processInputValue(cfg, cfg.default);
        }

        return result;
    }

    #processInputValue(cfg, value) {
        const name = cfg.name;
        let result = null;

        switch (cfg.type) {
            case 'boolean':
                if (typeof value === 'boolean') {
                    result = value;
                } else if (cfg.enum === null) {
                    switch (value) {
                        case 'true':
                        case 'on':
                        case 'enable':
                        case '1':
                        case 'active':
                        case 'yes':
                            result = true;
                            break;

                        case 'false':
                        case '0':
                        case 'off':
                        case 'disable':
                        case 'inactive':
                        case 'no':
                            result = false;
                            break;
                        default:
                            throw new Error(`Invalid enum value for parameter '${name}' : ${value}`);
                    }
                } else {
                    result = (value !== cfg.enum[0]); // 0th element is false
                }
                break;

            case 'number':
                if (cfg.enum === null) {
                    const x = Number.parseFloat(value);
                    if (Number.isNaN(x)) {
                        throw new Error(`Non-numeric value for numeric parameter '${name}'`)
                    }

                    result = x;
                } else {
                    result = cfg.enum.indexOf(value);
                }
                break;

            case 'string':
                if (cfg.enum !== null && !cfg.enum.includes(value)) {
                       throw new Error(`Invalid enum value for parameter '${name}' : ${value}`)
                } else {
                    result = value;
                }
                break;

            default:
                throw new Error('Invalid type');
        }

        return result;
    }
}

BullfrogCommon.deepfreeze(FrogOpts);
