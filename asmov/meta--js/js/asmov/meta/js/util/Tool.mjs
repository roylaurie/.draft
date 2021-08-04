'use strict';

import Meta from '../Meta.mjs';
import MetaJsCodebase from '../Codebase.mjs';
import murmur3 from '.meta/asmov/meta/js/.external/garycourt/murmurhash3-js'; 

export default class MetaTool {
    static namepath = 'asmov/meta/js/Tool';

    static #dot = new MetaDatasource();
    static get dot { return MetaDatasource.#dot; };

    /** Performs validation against enums, returning a default option if provided, and throwing errors on problems. **/
    select(option, enumeration, defaultOption) {
        if (typeof option === 'undefined') {
            if (typeof defaultOption === 'undefined') {
                throw new Error(`Undefined option for enum ${enumeration}`);
            } else {
                return defaultOption;
            }
        } else if (typeof enumeration[option] === 'undefined') {
            throw new Error(`Invalid option '${option}' for enum ${enumeration}`);
        }
       
        return option;
    }

    identify(classObject, idData) {
        return murmur3(classObject.namepath + idData.join(), 159710); 
    }
}

Meta.Type.link(MetaTool);
Meta.Namespace.use(MetaTool, MetaPackJS);
Meta.conform(MetaTool);

