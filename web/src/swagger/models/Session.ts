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
import type { SystemTime } from './SystemTime';
import {
    SystemTimeFromJSON,
    SystemTimeFromJSONTyped,
    SystemTimeToJSON,
} from './SystemTime';

/**
 * 
 * @export
 * @interface Session
 */
export interface Session {
    /**
     * set during login.
     * 
     * description for a session. clients can set this to whatever they want, or let the user set it themselves.
     * 
     * something like time and client name can be useful. just make sure they are human readable!
     * @type {any}
     * @memberof Session
     */
    description?: any | null;
    /**
     * timestamp when the token expires. it will not be usable anymore once the expiry date has passed.
     * @type {SystemTime}
     * @memberof Session
     */
    expiresAt: SystemTime;
    /**
     * session secret! put this in the header called 'Key'
     * @type {any}
     * @memberof Session
     */
    secret: any | null;
    /**
     * unique id for a user, uses uuid v7. this is based on time and makes ordering them in the databse faster.
     * @type {any}
     * @memberof Session
     */
    userId: any | null;
}

/**
 * Check if a given object implements the Session interface.
 */
export function instanceOfSession(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "expiresAt" in value;
    isInstance = isInstance && "secret" in value;
    isInstance = isInstance && "userId" in value;

    return isInstance;
}

export function SessionFromJSON(json: any): Session {
    return SessionFromJSONTyped(json, false);
}

export function SessionFromJSONTyped(json: any, ignoreDiscriminator: boolean): Session {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'description': !exists(json, 'description') ? undefined : json['description'],
        'expiresAt': SystemTimeFromJSON(json['expires_at']),
        'secret': json['secret'],
        'userId': json['user_id'],
    };
}

export function SessionToJSON(value?: Session | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'description': value.description,
        'expires_at': SystemTimeToJSON(value.expiresAt),
        'secret': value.secret,
        'user_id': value.userId,
    };
}
