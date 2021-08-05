#!/usr/bin/env node
'use strict';

import FrogOpts from '@asmov/bullfrog/common/FrogOpts'

const options = {
    a: { name: 'app', type: 'string', default: 'bullfrog' },
    c: { name: 'config', type: 'string', valid: 'token', default: 'default' },
    j: { name: 'json', type: 'boolean', alias: 'output', default: 'json' },
    o: { name: 'output', type: 'string', valid: 'token', default: 'term' },
    r: { name: 'remote', type: 'string' },
    x: { name: 'debug', type: 'boolean' },
    X: { name: 'debugall', type: 'boolean' },
};

const parameters = {
    'bool.bar': { valid: 'boolean', enum: ['on','off'] },
    'num.bar': { valid: 'integer', enum: ['first','second','third'] },
    'enum.bar': { valid: 'enum', enum: ['first','second','third'], default: 'second' },
    'cat.dog': { required: true },
    'kitten': { required: true, position: 1 },
    'cat': { required: true, position: 2 },
    'puppy': { position: 3, default: 'poodle' },
    'dog': { position: 4 },
};

const frogopts = new FrogOpts(options, parameters);

const cmdline_argv = frogopts.parseArguments(process.argv, 2);
console.log('cmdline argv', cmdline_argv);

const cmdline_shlex = frogopts.parseCommandline( process.argv.join(' '), 2);
console.log('cmdline shlex', cmdline_shlex);

console.log('cmdline json', JSON.stringify(cmdline_argv));
