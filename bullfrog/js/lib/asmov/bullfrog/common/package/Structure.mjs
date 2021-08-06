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
            filepath: '/README.md'
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
        makepath: (fs, module, lang) => `${fs.package[lang].module.path}/${module.split('.','/')}`,
        lib: {
            makepath: (fs, module, lang) => fs.module.path + '/lib',
        }
    };

    static operation = {
        makefilepath: (fs, operation, lang) => {
            let ext = null;
            switch (lang) {
                case 'bash':
                    ext = '.module.bash'
                    break;
                case 'js':
                    ext = '.module.mjs';
                    break;
            }

            return fs.module.path + '/' + operation + ext;
        }
    };

    static build(template, basepath, fs, ...args) {
        return this.#structure({}, template, basepath, fs, ...args);
    }

    static #structure(structure, template, basepath, fs, ...args) {
        for (const key in template) {
            const value = template[key];
            switch (key) {
                case 'path':
                case 'filepath':
                    structure[key] = basepath + value;
                    break;

                case 'makepath':
                case 'makefilepath':
                    if (!(value instanceof Function)) {
                        throw new Error('Expected function for path generator');
                    }

                    structure[(key === 'makepath' ? 'path' : 'filepath')] = basepath + '/' + value(fs, ...args);
                    break;

                default:
                    structure[key] = {};
                    this.#structure(structure[key], value, basepath, fs, ...args)
            }
        }

        return structure;
    }
}

Common.deepfreeze(BullfrogPackageStructure);

