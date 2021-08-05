'use strict';

import { expect } from 'chai'
import FrogOpts from '@asmov/bullfrog/common/FrogOpts';
import shlex from 'shlex';

describe(FrogOpts.namepath, () => {
    function shlex_split(cmdline) {
        return shlex.split(cmdline);
    }

    describe('process', () => {
        const s = {
            passive: 'passive',
            defaults: 'defaults',
            simple: 'simple',
            few: 'few'
        };
        const optionCfg = {
            passive: [
                { flag: 's', name: 'flag.string', type: 'string' },
                { flag: 'b', name: 'flag.boolean', type: 'boolean' },
                { flag: 'n', name: 'flag.number', type: 'number' },
                { flag: 't', name: 'flag.enum.string', type: 'string', enum: ['fzero', 'fone', 'ftwo' ]},
                { flag: 'o', name: 'flag.enum.boolean', type: 'boolean', enum: ['ffail', 'fwin' ] },
                { flag: 'u', name: 'flag.enum.number', type: 'number', enum: ['fcero', 'funo', 'fdos' ] }
            ],
            defaults: [
                { flag: 's', name: 'flag.string', type: 'string', default: 'flag string' },
                { flag: 'b', name: 'flag.boolean', type: 'boolean', default: true },
                { flag: 'n', name: 'flag.number', type: 'number', default: 128 },
                { flag: 't', name: 'flag.enum.string', type: 'string', enum: ['fzero', 'fone', 'ftwo' ], default: 'ftwo'},
                { flag: 'o', name: 'flag.enum.boolean', type: 'boolean', enum: ['ffail', 'fwin' ], default: 'ffail' },
                { flag: 'u', name: 'flag.enum.number', type: 'number', enum: ['fcero', 'funo', 'fdos'], default: 'fcero' }
            ]
        };
        const settings = {
            simple: []
        };
        const module = {
            simple: 'test.module'
        };
        const operation = {
            simple: 'test.operation'
        }
        const parameterCfg = {
            passive: [
                { name: 'param.string', type: 'string' },
                { name: 'param.boolean', type: 'boolean' },
                { name: 'param.number', type: 'number' },
                { name: 'param.enum.string', type: 'string', enum: ['zero','one','two'] },
                { name: 'param.enum.boolean', type: 'boolean', enum: ['fail','win'] },
                { name: 'param.enum.number', type: 'number', enum: ['cero','uno','dos'] }
            ],
            defaults: [
                { name: 'param.string', type: 'string', default: 'string param' },
                { name: 'param.boolean', type: 'boolean', default: false },
                { name: 'param.number', type: 'number', default: 32 },
                { name: 'param.enum.string', type: 'string', enum: ['zero','one','two'], default: 'one' },
                { name: 'param.enum.boolean', type: 'boolean', enum: ['fail', 'win'], default: 'win' },
                { name: 'param.enum.number', type: 'number', enum: ['cero','uno','dos'], default: 'dos' }
            ]
        };
        const args = {
            simple: shlex_split('test.module test.operation'),
            few: shlex_split('-bs hello -u funo test.module test.operation --param.boolean true --param.string "foo bar" --param.enum.number uno')
        };
        const optionResults = {
            few: { 'flag.string': 'hello', 'flag.boolean': true, 'flag.enum.number': 1 },
            simple: { },
            defaults: {
                'flag.string': 'flag string', 'flag.boolean': true, 'flag.number': 128,
                'flag.enum.string': 'ftwo', 'flag.enum.boolean': false, 'flag.enum.number': 0
            }
        }
        const parameterResults = {
            few: {'param.boolean': true, 'param.string': 'foo bar', 'param.enum.number': 1 },
            defaults: {
                'param.string': 'string param', 'param.boolean': false, 'param.number': 32,
                'param.enum.string': 'one', 'param.enum.boolean': true, 'param.enum.number': 2
            }
        };

        function test_process(optionCfgKey, settingsKey, moduleKey, operationKey, parameterCfgKey, argsKey) {
            const frogopts = new FrogOpts(optionCfg[optionCfgKey], settings[settingsKey]);
            return frogopts.process(args[argsKey], (_module, _operation) => {
                expect(_module).to.equal(module[moduleKey]);
                expect(_operation).to.equal(operation[operationKey]);
                return parameterCfg[parameterCfgKey];
            });
        }

        it('returns without parameters or options when none are required, default, or provided', () => {
            const result = test_process(s.passive, s.simple, s.simple, s.simple, s.passive, s.simple);
            expect(result.options).to.be.empty;
            expect(result.module).to.equal(module.simple);
            expect(result.operation).to.equal(operation.simple);
            expect(result.parameters).to.be.empty;
        });

        it('returns with only parameters and options specified when none are required or default', () => {
            const result = test_process(s.passive, s.simple, s.simple, s.simple, s.passive, s.few);
            expect(result.options).to.deep.equal(optionResults.few);
            expect(result.module).to.equal(module.simple);
            expect(result.operation).to.equal(operation.simple);
            expect(result.parameters).to.deep.equal(parameterResults.few);
        });

        it('returns with all default parameters and options when none are specified', () => {
            const result = test_process(s.defaults, s.simple, s.simple, s.simple, s.defaults, s.simple);
            expect(result.options).to.deep.equal(optionResults.defaults);
            expect(result.module).to.equal(module.simple);
            expect(result.operation).to.equal(operation.simple);
            expect(result.parameters).to.deep.equal(parameterResults.defaults);
        });

        it('returns with default parameters and options overridden where specified', () => {
            const result = test_process(s.defaults, s.simple, s.simple, s.simple, s.defaults, s.few);
            const expectedOptions = { ...optionResults.defaults, ...optionResults.few }
            const expectedParameters = { ...parameterResults.defaults, ...parameterResults.few }
            expect(result.options).to.deep.equal(expectedOptions);
            expect(result.module).to.equal(module.simple);
            expect(result.operation).to.equal(operation.simple);
            expect(result.parameters).to.deep.equal(expectedParameters);
        });

        it('throws error when the module is not provided');
        it('throws error when the module is unknown');
        it('throws error when the operation is unknown');
        it('returns the default operation when one is not provided');
        it('throws error when required options are not provided');
        it('throws error when required parameters are not provided');
        it('throws error when required positional parameters are not provided');
        it('throws error when an invalid boolean option is provided');
        it('throws error when an invalid number option is provided');
        it('throws error when an invalid enum option is provided');
        it('throws error when an invalid boolean parameter is provided');
        it('throws error when an invalid number parameter is provided');
        it('throws error when an invalid enum parameter is provided');
        it('throws error on undefined parameter config field');
        it('throws error on missing parameter config fields');
        it('throws error on wrong type or value for parameter config fields');
        it('throws error when default and required are both enabled in parameter configs');
    })

    describe('construct', () => {
        it('throws error on undefined option config field');
        it('throws error on missing option config fields');
        it('throws error on wrong type or value for option config fields');
        it('throws error when default and required are both enabled in option configs');
        it('throws error on undefined setting field');
        it('throws error on wrong type or value for setting field');
    });
})