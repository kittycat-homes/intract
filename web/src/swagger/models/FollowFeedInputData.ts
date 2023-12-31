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
 * @interface FollowFeedInputData
 */
export interface FollowFeedInputData {
    /**
     * only show this feed and items from it if show hidden is enabled
     * @type {any}
     * @memberof FollowFeedInputData
     */
    hide: any | null;
    /**
     * the uri for your feed
     * @type {any}
     * @memberof FollowFeedInputData
     */
    uri: any | null;
}

/**
 * Check if a given object implements the FollowFeedInputData interface.
 */
export function instanceOfFollowFeedInputData(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "hide" in value;
    isInstance = isInstance && "uri" in value;

    return isInstance;
}

export function FollowFeedInputDataFromJSON(json: any): FollowFeedInputData {
    return FollowFeedInputDataFromJSONTyped(json, false);
}

export function FollowFeedInputDataFromJSONTyped(json: any, ignoreDiscriminator: boolean): FollowFeedInputData {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'hide': json['hide'],
        'uri': json['uri'],
    };
}

export function FollowFeedInputDataToJSON(value?: FollowFeedInputData | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'hide': value.hide,
        'uri': value.uri,
    };
}

