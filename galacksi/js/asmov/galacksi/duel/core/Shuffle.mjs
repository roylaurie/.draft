'use strict';

export default class Shuffle {
    static randomOption(options) {
        if (options.length === 1) {
            return options[0];
        }

        return options[Math.floor(Math.random() * Math.floor(options.length))];
    }
}


