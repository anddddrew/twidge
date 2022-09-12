// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Operations = {
    queries: 
        { key: ["version"], result: string } | 
        { key: ["settings.get", GetSettingsArgs], result: Settings | null },
    mutations: 
        { key: ["settings.set", SetSettingsArgs], result: Settings },
    subscriptions: never
};

export interface GetSettingsArgs { key: string }

export interface Settings { id: number, name: string, value: string, createdAt: string, updatedAt: string }

export interface SetSettingsArgs { key: string, value: string }