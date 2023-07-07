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
 * data used to log a user in
 * @export
 * @interface LoginData
 */
export interface LoginData {
    /**
     * description for a session. clients can set this to whatever they want, or let the user set it themselves.
     * 
     * something like time and client name can be useful. just make sure they are human readable!
     * @type {any}
     * @memberof LoginData
     */
    description?: any | null;
    /**
     * password for the user you want to log in
     * @type {any}
     * @memberof LoginData
     */
    password: any | null;
    /**
     * username for the account you want to log in
     * @type {any}
     * @memberof LoginData
     */
    username: any | null;
}

/**
 * Check if a given object implements the LoginData interface.
 */
export function instanceOfLoginData(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "password" in value;
    isInstance = isInstance && "username" in value;

    return isInstance;
}

export function LoginDataFromJSON(json: any): LoginData {
    return LoginDataFromJSONTyped(json, false);
}

export function LoginDataFromJSONTyped(json: any, ignoreDiscriminator: boolean): LoginData {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'description': !exists(json, 'description') ? undefined : json['description'],
        'password': json['password'],
        'username': json['username'],
    };
}

export function LoginDataToJSON(value?: LoginData | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'description': value.description,
        'password': value.password,
        'username': value.username,
    };
}

