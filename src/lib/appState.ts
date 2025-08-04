/**
 * @file Manages the core application lifecycle state.
 */

import { writable } from "svelte/store";

/**
 * Represents the possible states of the application's lifecycle.
 */
export type AppState = "selecting_vault" | "loading" | "ready" | "error";

/**
 * Defines the shape of the application status, allowing for an optional error message.
 */
export interface AppStatus {
    state: AppState;
    message?: string;
}

/**
 * Manages the application's current status.
 */
export const appStatus = writable<AppStatus>({ state: "selecting_vault" });
