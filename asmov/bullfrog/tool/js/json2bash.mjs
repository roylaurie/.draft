#!/usr/bin/env node
'use strict'; // ecma.version 2021

import fs from 'fs/promises';
import util from 'util';
import process from 'process';

import Ajv from 'ajv';

const SCHEMA_NAME="namespace";

async function loadSchema(name, filepath) {
    const jsonSchema = JSON.parse((await fs.readFile(filepath)).toString());

    // load json schema into ajv
    const ajv = new Ajv();
    ajv.addSchema(jsonSchema, name);
    return ajv;
}

async function validateData(ajv, name, filepath) {
    const jsonData = JSON.parse((await fs.readFile(filepath)).toString());
    const validateFunc = ajv.getSchema(name);
    if (!validateFunc(jsonData)) {
        throw new Error("Unable to parse invalid JSON. " + inspect(validateFunc.errors));
    }

    return jsonData;
}

function buildKV(data) {
    // define a key-value array that accepts either strings or arrays of strings as values. namespace is key.
    const kv = {};

    // walk json schema object. for each definition
    //   compile the namespace string for each definition context
    //   store the appropriate value (string or array<string>) for the definition by its namespace

    const packageNamespace = data.package.name;
    const packagePrefix = 'package.' + packageNamespace  + '.';
    kv[packagePrefix + 'namespaces'] = Object.keys(data.package.namespaces);

    for (const namespace in data.package.namespaces) {
        const namespaceData = data.package.namespaces[namespace];
        const namespacePrefix = packagePrefix + 'namespaces.' + namespace + '.';
        kv[namespacePrefix + 'desc'] = namespaceData.desc;
        kv[namespacePrefix + 'operations'] = Object.keys(namespaceData.operations);

        for (const operation in namespaceData.operations) {
            const operationData = namespaceData.operations[operation];
            const opPrefix = namespacePrefix + 'operations.' + operation + '.';
            kv[opPrefix + 'lang'] = operationData.lang;
            kv[opPrefix + 'desc'] = operationData.desc;
            kv[opPrefix + 'outputs'] = ( typeof operationData.outputs !== 'undefined' ? operationData.outputs : [] );
            kv[opPrefix + 'parameters'] = Object.keys(operationData.parameters);

           for (const parameter in operationData.parameters) {
                const parameterData = operationData.parameters[parameter];
                const paramPrefix = opPrefix + 'parameters.' + parameter + '.';
                kv[paramPrefix + 'desc'] = parameterData.desc;
                kv[paramPrefix + 'required'] = parameterData.required.toString();

                if (typeof parameterData.position !== 'undefined') {
                    kv[paramPrefix + 'position'] = parameterData.position.toString();
                }
                if (typeof parameterData.default !== 'undefined') {
                    kv[paramPrefix + 'default'] = parameterData.default;
                }
                if (typeof parameterData.enum !== 'undefined') {
                    kv[paramPrefix + 'enum'] = parameterData.enum;
                }
            }
        }
    }

    return kv;
}

function printBashCfg(kv) {
    const lines = [];

    // for each key-value pair: write a bash config statement to the bash output file, based on type
    for (const key in kv) {
        const value = kv[key];
        const type = typeof value === 'string' ? 'string' : 'array';
        let line = 'frogcfg_set_key ' + type + ' "' + key + '"';

        if (type === 'array') {
            for (const val of value) {
                line += ' "' + val + '"';
            }
        } else { // string
            line += ' "' + value + '"';
        }

        console.log(line);
    }
}

function printBashHeader() {
    console.log('#!/bin/bash\nset -o allexport -o errexit -o privileged -o pipefail -o nounset\n');
}

function printBashFooter(packageNamespace) {
    console.log('\nFROG_PACKAGE_NAMESPACE="' + packageNamespace + '"');
}

function inspect(obj) {
    return util.inspect(obj, false, null, true);
}

function logobj(subject, obj) {
    console.log(subject, util.inspect(obj, false, null, true));
}

async function main() {
    if (process.argv.length < 4) {
        console.log('Usage: json2bash.mjs <json schema filepath> <json data filepath>')
        process.exit(1);
        return;
    }

    const jsonSchemaFilepath = process.argv[2];
    const jsonDataFilepath = process.argv[3];

    // validate json data against ajv
    const ajv = await loadSchema(SCHEMA_NAME, jsonSchemaFilepath);
    const data = await validateData(ajv, SCHEMA_NAME, jsonDataFilepath);

    const packageNamespace = data.package.name;
    // build the key-value config based on data
    const kv = buildKV(data);

    // write bash src to output
    printBashHeader();
    printBashCfg(kv);
    printBashFooter(packageNamespace);
}

await main();