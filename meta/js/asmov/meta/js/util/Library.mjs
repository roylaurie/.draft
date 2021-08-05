'use strict';

export default class Library {
    static namepath = 'asmov/meta/js/Library';

    static dot = new Library();

    static #keys = { namepath: 'namepath' };

    #definitions = new Map();

    #reserved = [
        'asmov/meta/js/Trait',
        'asmov/meta/js/Type',
        'asmov/meta/js/Namespace',
        'asmov/meta/js/Interface',
        'asmov/meta/js/Model'
    ];
    
    #secrets = new Map();

    constructor() {
        if (typeof Library.dot !== 'undefined') {
            throw new Error('MetaLibrary already initialized.');
        }
    }

    reserve(secret, metatrait) {
        if (typeof metatrait[Library.#keys.namepath] !== 'string') {
            throw new Error(`Invalid metatrait ${metatrait}`);
        }
        
        const namepath = metatrait[Library.#keys.namepath];
        if (this.#definitions.has(secret)) {
            throw new Error(`Trait ${namepath} already linked to Library`); // shouldn't happen
        }

        this.#definitions.set(secret, new Map());
    }

    link(secret, metatype) {
        if (typeof metatype[Library.#keys.namepath] !== 'string') {
            throw new Error(`Invalid metatype ${metatype}`);
        }
        
        const namepath = metatype[Library.#keys.namepath];
        const definitionMap = this.#definitions.get(secret);
        if (typeof definitionMap === 'undefined') {
            throw new Error(`Unknown secret for Library`);
        }

        definitionMap.set(namepath, new LibraryDefinition(secret, namepath, metatype));
    }

    decorate(secret, metatype, metatrait, traitsecret) {
        if (typeof metatype[Library.#keys.namepath] !== 'string' || typeof metatrait[Library.#keys.namepath] !== 'string') {
            throw new Error(`Invalid metatype ${metatype} or metatrait ${metatrait}`);
        }
        
        const definitionMap = this.#definitions.get(secret);
        if (typeof definitionMap === 'undefined') {
            throw new Error(`Unknown secret for Library`);
        }

        const definition = definitionMap.get(metatype.namepath);
        definition.decorate(secret, new LibraryDefinition(traitsecret, metatrait.namepath, metatrait));
    }
}

class LibraryDefinition {
    #decorators = new Map();
    #secret = null;
    #namepath = null;
    #metatype = null;

    constructor(secret, namepath, metatype) {
        this.#secret = secret;
        this.#namepath = namepath;
        this.#metatype = metatype;
    }

    namepath() { return this.#namepath; }
    metatype() { return this.#metatype; }
    decorators() { return this.#decorators; }

    decorate(secret, definition) {
        if (secret !== this.#secret) {
            throw new Error('Not authorized to modify Library Definition');
        }

        this.#decorators.set(definition.namepath(), definition);
    }
}


