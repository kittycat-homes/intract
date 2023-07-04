/* tslint:disable */
/* eslint-disable */
/**
 * intract
 * intract is an rss reader for the modern web
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
 * data needed to sign up for an account
 * @export
 * @interface RegisterData
 */
export interface RegisterData {
    /**
     * give a bit of context as to why you want to join. links to social media and a little bit about youself can provide good reasons for an admin to let you join.
     * @type {any}
     * @memberof RegisterData
     */
    joinReason?: any | null;
    /**
     * password to use. it has to be at least as long as the password set in the server config.
     * @type {any}
     * @memberof RegisterData
     */
    password: any | null;
    /**
     * username to use, you can change this later. this is used for logging in.
     * @type {any}
     * @memberof RegisterData
     */
    username: any | null;
}

/**
 * Check if a given object implements the RegisterData interface.
 */
export function instanceOfRegisterData(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "password" in value;
    isInstance = isInstance && "username" in value;

    return isInstance;
}

export function RegisterDataFromJSON(json: any): RegisterData {
    return RegisterDataFromJSONTyped(json, false);
}

export function RegisterDataFromJSONTyped(json: any, ignoreDiscriminator: boolean): RegisterData {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'joinReason': !exists(json, 'join_reason') ? undefined : json['join_reason'],
        'password': json['password'],
        'username': json['username'],
    };
}

export function RegisterDataToJSON(value?: RegisterData | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'join_reason': value.joinReason,
        'password': value.password,
        'username': value.username,
    };
}
