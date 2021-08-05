'use strict';

import Meta from '../../../../ext/asmov--meta--js/Meta.mjs';
import MetaProduct from '../../../../ext/asmov--meta--js/Product.mjs';
import GalacksiDuelProduct from '../../../asmov/galacksi/duel/core/GalacksiDuelMetaProduct.mjs';

export default class GalacksiDuelCoreProduct extends MetaProduct {
    static namepath = 'asmov/galacksi/duel/core/MetaProduct';
    static namespace = 'asmov/galacksi/duel/core/';

    constructor() {
        super({
            MetaProduct.dataKeys.namespace:     'asmov/galacksi/duel/core/',
            MetaProduct.dataKeys.parent:        GalacksiDuelProduct,
            MetaProduct.dataKeys.version:       '2021.0.0',
            MetaProduct.dataKeys.organisation:  'Asmov LLC',
            MetaProduct.dataKeys.copyright:     '2021',

            MetaProduct.dataKeys.pubkey:        0x5F1DF16B2B704C8A578D0BBAF74D385CDE12C11EE50455F3C438EF4C3FBCF649B6DE611FEAE06279A60939E028A8D65C10B73071A6F16719274855FEB0FD8A6704  
        });
    }
}

Meta.linkProduct(GalacksiDuelCoreProduct);

