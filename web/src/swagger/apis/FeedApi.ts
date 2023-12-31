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


import * as runtime from '../runtime';
import type {
  FollowFeedInputData,
} from '../models';
import {
    FollowFeedInputDataFromJSON,
    FollowFeedInputDataToJSON,
} from '../models';

export interface FollowFeedRequest {
    followFeedInputData: FollowFeedInputData;
}

export interface ShowFeedsRequest {
    showHidden: any;
}

/**
 * 
 */
export class FeedApi extends runtime.BaseAPI {

    /**
     * follow a new feed
     * follow a feed
     */
    async followFeedRaw(requestParameters: FollowFeedRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters.followFeedInputData === null || requestParameters.followFeedInputData === undefined) {
            throw new runtime.RequiredError('followFeedInputData','Required parameter requestParameters.followFeedInputData was null or undefined when calling followFeed.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/authorized/feed/follow`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: FollowFeedInputDataToJSON(requestParameters.followFeedInputData),
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * follow a new feed
     * follow a feed
     */
    async followFeed(requestParameters: FollowFeedRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.followFeedRaw(requestParameters, initOverrides);
    }

    /**
     * show feeds followed by the authenticated user
     * show feeds
     */
    async showFeedsRaw(requestParameters: ShowFeedsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<any>> {
        if (requestParameters.showHidden === null || requestParameters.showHidden === undefined) {
            throw new runtime.RequiredError('showHidden','Required parameter requestParameters.showHidden was null or undefined when calling showFeeds.');
        }

        const queryParameters: any = {};

        if (requestParameters.showHidden !== undefined) {
            queryParameters['show_hidden'] = requestParameters.showHidden;
        }

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/authorized/feed/show/`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        if (this.isJsonMime(response.headers.get('content-type'))) {
            return new runtime.JSONApiResponse<any>(response);
        } else {
            return new runtime.TextApiResponse(response) as any;
        }
    }

    /**
     * show feeds followed by the authenticated user
     * show feeds
     */
    async showFeeds(requestParameters: ShowFeedsRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<any> {
        const response = await this.showFeedsRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
