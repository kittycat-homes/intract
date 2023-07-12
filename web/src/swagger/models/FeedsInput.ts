/* tslint:disable */
/* eslint-disable */
/**
 * intract
 * intract is an rss reader and generator for the modern web
 *
 * The version of the OpenAPI document: 0.0.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
/**
 * 
 * @export
 * @interface FeedsInput
 */
export interface FeedsInput {
    /**
     * show feeds that you have hidden
     * @type {any}
     * @memberof FeedsInput
     */
    showHidden: any | null;
}

/**
 * Check if a given object implements the FeedsInput interface.
 */
export function instanceOfFeedsInput(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "showHidden" in value;

    return isInstance;
}

export function FeedsInputFromJSON(json: any): FeedsInput {
    return FeedsInputFromJSONTyped(json, false);
}

export function FeedsInputFromJSONTyped(json: any, ignoreDiscriminator: boolean): FeedsInput {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'showHidden': json['show_hidden'],
    };
}

export function FeedsInputToJSON(value?: FeedsInput | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'show_hidden': value.showHidden,
    };
}
