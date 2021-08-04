'use strict';

import MetaTrait from '.meta/asmov/meta/js/Trait.mjs';
import MetaType from '.meta/asmov/meta/js/Type.mjs';
import MetaModel from '.meta/asmov/meta/js/Model.mjs';

/* TODO:
Template replacement terms:
  _CLASS_NAME_ The class name. E.g., SimpleTrait
  _NAMEPATH_ The NAMEPATH of this class. E.g., myorg/stuff/simple/Trait
  _TRAIT_NAME_ The trait name for the class. E.g., Simple
  _LINKTYPE_ The term to refer to types of this trait. E.g., metatype
  __LINKTYPE_S_ The name of the variable to store the traits linked in. E.g., metatypes
*/

export default class _CLASS_NAME_ {
    static namepath = '_NAMEPATH_';
    static traitname = '_TRAIT_NAME_';

    static staticTraits = {};
    static staticMethodTraits = {};
    static methodTraits = {};
    static dataTraits = {};

    static init() {}

    static confirm(_LINKTYPE_) {
        MetaType.confirm(_LINKTYPE_);
        MetaModel.confirm(_LINKTYPE_);
        MetaTrait.confirmTrait(_LINKTYPE_, _CLASS_NAME_);
    }

    static link(_LINKTYPE_) { }

    static linked(_LINKTYPE_) { }

    static confirmLink(nametype) { }

    static conform(_LINKTYPE_) {
        this.confirm(_LINKTYPE_);
        this.confirmLink(_LINKTYPE_);
    }

    static get(_NAMEPATH_) {}

    namepath = _CLASS_NAME_.namepath;

    constructor() { throw new Error('_CLASS_NAME_ cannot be initialized.'); }
}

