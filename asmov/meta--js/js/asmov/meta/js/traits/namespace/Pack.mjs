'use strict';

import Meta from '../Meta.mjs';

export default class Pack {
    static namepath = 'asmov/meta/js/codebase/Pack';

    static staticTraits = {};
    static staticMethodTraits = {}
    static methodTraits = {};
    static dataTraits = {
        namespace: 'namespace',
        version: 'version',
        organisation: 'organisation',
        copyright: 'copyright',
        license: 'license',
        pubkey: 'pubkey',
        product: 'product',
        codebase: 'codebase',
        subpacks: 'subpacks',
        platform: 'plastform',
    };

    static languages = {
        js: 'js',
        rust: 'rust',
        python: 'python',
        java: 'java',
        bash: 'bash'
    };

    static platforms = {
        js: 'js',
        rust: 'rust',
        python: 'python',
        java: 'java',
        nodejs: 'nodejs',
        browser: 'browser',
        electron: 'electron',
        dapp: 'dapp',
        linux: 'linux',
        osx: 'osx',
        windows: 'windows',
        android: 'android',
        ios: 'ios',
        xbox: 'xbox',
        playstation: 'playstation',
        nintendo: 'nintendo',
    };
        
    static architectures = {
        any: 'any',
        x32: 'x32',
        x64: 'x64',
        arm32: 'arm32',
        arm64: 'arm64'
    };

    static from(data, datasource) {
        return new Pack(data);
    }

    #namespace = null;
    #version = null;
    #organisation = null;
    #copyright = null;
    #license = null;
    #pubkey = null;
    #id = null;

    constructor(data) {
        this.#namespace = data[MetaPackage.dataTraits.namespace];
        this.#version = data[MetaPackage.dataTraits.version];
        this.#organisation = data[MetaPackage.dataTraits.organisation];
        this.#copyright = data[MetaPackage.dataTraits.copyright];
        this.#license = data[MetaPackage.dataTraits.license];
        this.#pubkey = data[MetaPackage.dataTraits.pubkey];
        this.#id = data[Meta.Model.dataTraits.id] || ModelMeta.id32(this, [this.#namespace, this.#version]);
    }

    id() {
        return this.#id;
    }

    data() {
        return Object.freeze({
            Meta.Model.dataTraits.namepath = Pack.namepath,
            Meta.Model.dataTraits.id = this.#id,
            MetaPackage.dataTraits.namespace: this.#namespace,
            MetaPackage.dataTraits.version: this.#version,
            MetaPackage.dataTraits.organisation: this.#organisation,
            MetaPackage.dataTraits.copyright: this.#copyright,
            MetaPackage.dataTraits.license: this.#license,
            MetaPackage.dataTraits.pubkey: this.#pubkey
        });
    }
}
