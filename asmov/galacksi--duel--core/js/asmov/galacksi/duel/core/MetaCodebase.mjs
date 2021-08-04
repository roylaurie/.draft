'use strict';

import Meta from '../../../../asmov--meta--js/Meta.mjs';
import MetaPack from '../../../../asmov--meta--js/Pack.mjs';
import GalacksiDuelCoreProduct from '../../../asmov/galacksi/duel/core/GalacksiDuelCoreMetaProduct.mjs';

export default class GalacksiDuelCorePack extends MetaPack {
    static namepath = 'asmov/galacksi/duel/core/MetaPack';
    static namespace = 'asmov/galacksi/duel/core/';

    static packaging() {
        Meta.pack(GalacksiDuelCorePack).packaging();
    };

    packaging() {
        console.log(JSON.stringify(this.data()));
        process.exit(0);
    }

    constructor() {
        super({
            MetaPack.dataKeys.codebase:      'asmov--galacksi--duel--core',
            MetaPack.dataKeys.namespace:     'asmov/galacksi/duel/core/',
            MetaPack.dataKeys.version:       '2021.1.0',
            MetaPack.dataKeys.organisation:  'Asmov LLC',
            MetaPack.dataKeys.platform:      MetaPack.platform.js,
            MetaPack.dataKeys.architecture:  MetaPack.architectures.any,
            MetaPack.dataKeys.language:      MetaPack.languages.js,
            MetaPack.dataKeys.copyright:     '2021',
            MetaPack.dataKeys.license:       MetaPack.license.GPLv3,
            MetaPack.dataKeys.product:       GalacksiDuelCoreProduct,

            MetaPack.dataKeys.pubkey:        0x5F1DF16B2B704C8A578D0BBAF74D385CDE12C11EE50455F3C438EF4C3FBCF649B6DE611FEAE06279A60939E028A8D65C10B73071A6F16719274855FEB0FD8A6704  

            MetaPack.dataKeys.subpacks = [
                'asmov/galacksi/duel/core/net/'
            ];
        });
    }

    
}

Meta.linkPack(GalacksiDuelPack);
