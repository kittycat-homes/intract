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
 * 
 * @export
 * @interface User
 */
export interface User {
    /**
     * bio of this user
     * @type {any}
     * @memberof User
     */
    bio?: any | null;
    /**
     * should be more prominently featured than username
     * @type {any}
     * @memberof User
     */
    displayName?: any | null;
    /**
     * unique id for a user, uses uuid v7. this is based on time and makes ordering them in the databse faster.
     * @type {any}
     * @memberof User
     */
    id: any | null;
    /**
     * 
     * @type {any}
     * @memberof User
     */
    powerlevel: any | null;
    /**
     * pronouns of this user, can be set back to none
     * @type {any}
     * @memberof User
     */
    pronoun?: any | null;
    /**
     * username for logging in
     * @type {any}
     * @memberof User
     */
    username: any | null;
}

/**
 * Check if a given object implements the User interface.
 */
export function instanceOfUser(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "id" in value;
    isInstance = isInstance && "powerlevel" in value;
    isInstance = isInstance && "username" in value;

    return isInstance;
}

export function UserFromJSON(json: any): User {
    return UserFromJSONTyped(json, false);
}

export function UserFromJSONTyped(json: any, ignoreDiscriminator: boolean): User {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'bio': !exists(json, 'bio') ? undefined : json['bio'],
        'displayName': !exists(json, 'display_name') ? undefined : json['display_name'],
        'id': json['id'],
        'powerlevel': json['powerlevel'],
        'pronoun': !exists(json, 'pronoun') ? undefined : json['pronoun'],
        'username': json['username'],
    };
}

export function UserToJSON(value?: User | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'bio': value.bio,
        'display_name': value.displayName,
        'id': value.id,
        'powerlevel': value.powerlevel,
        'pronoun': value.pronoun,
        'username': value.username,
    };
}

