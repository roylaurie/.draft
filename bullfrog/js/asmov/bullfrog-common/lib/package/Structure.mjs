'use strict';

import Common from '@asmov/bullfrog/common/Common';

export default class BullfrogPackageStructure {
    static package = {
        path: '',
        bash: {
            path: '/bash',
            bin: {
                path: '/bash/bin',
            },
            cfg: {
                path: '/bash/cfg',
            },
            heredoc: {
                path: '/bash/heredoc'
            },
            lib: {
                path: '/bash/lib',
            },
            module: {
                path: '/bash/module',
            }
        },
        js: {
            path: '/js',
            bin: {
                path: '/js/bin',
            },
            lib: {
               path: '/js/lib'
            },
            module: {
                path: '/js/module'
            }
        },
        json: {
            path: '/json',
            data: {
                path: '/json/data',
                package: {
                    config: {
                        filepath: '/json/data/package/modules.json',
                    }
                }
            },
            schema: {
                path: '/json/schema',
                package: {
                    config: {
                        filepath: '/json/schema/package/modules.schema.json',
                    }
                }
            }
        },
        etc: {
            path: '/etc',
            package: {
                path: '/etc/package',
                maintainer: {
                    filepath: '/etc/package/MAINTAINER.txt',
                },
                organization: {
                    filepath: '/etc/package/ORG.txt',
                }
            }
        },
        file: {
            path: '/file',
        },
        license: {
            filepath: '/COPYING.txt',
            summary: {
                filepath: '/LICENSE.txt'
            },
        },
    };

    static dev = {
        path: '',
        bash: {
            heredoc: {
                generator: {
                    path: '/bash/heredoc/gen'
                }
            }
        },
        doc: {
            path: '/doc'
        },
        test: {
            path: '/test',
            js: {
                path: '/test/js',
                tests: {
                    path: '/test/js/tests'
                },
            }
        },
        tool: {
            path: '/tool'
        },
        readme: {
            filepath: '/Notes.md'
        },
        npm: {
            config: {
                filepath: '/package.json',
                lock: {
                    filepath: '/package-lock.json'
                }
            }
        }
    };

    static module = {
        makepath: (basepath, module, fs, namespace, lang) => basepath + `/${lang}/module/`
            + namespace.split('.').slice(0, -1).join('/'),
        makefilepath: (basepath, module, fs, namespace, lang) => module.path + '/'
            + namespace.split('.').slice(-1).join() + '.module' + this.langext(lang),
    };

    static langext(lang) {
        switch(lang) {
            case 'js':
                return '.mjs';
            case 'bash':
                return '.bash';
            default:
                throw new Error('Uknown language: ' + lang)
        }
    }

    static build(template, basepath, fs, ...args) {
        const structure = {};
        return this.#structure(structure, structure, template, basepath, fs, ...args);
    }

    static #structure(tree, structure, template, basepath, fs, ...args) {
        if (typeof template['makepath'] === 'function') {
            structure.path = template.makepath(basepath, tree, fs, ...args);
        }
        if (typeof template['makefilepath'] === 'function') {
            structure.filepath = template.makefilepath(basepath, tree, fs, ...args);
        }

        for (const key in template) {
            const value = template[key];
            switch (key) {
                case 'path':
                case 'filepath':
                    structure[key] = basepath + value;
                    break;

                case 'makepath':
                case 'makefilepath':
                    break;

                default:
                    structure[key] = {};
                    this.#structure(tree, structure[key], value, basepath, fs, ...args)
            }
        }

        return structure;
    }
}

Common.deepfreeze(BullfrogPackageStructure);

