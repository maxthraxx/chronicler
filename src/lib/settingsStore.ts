/**
 * @file Manages persistent user settings using the Tauri Store Plugin.
 *
 * This store handles UI-related preferences that need to persist between
 * application sessions. It uses the LazyStore to avoid issues with
 * Server-Side Rendering (SSR) by only loading the data from disk when
 * it's first accessed.
 */

import { writable } from "svelte/store";
import { LazyStore } from "@tauri-apps/plugin-store";

// Define the shape of all your settings
export type Theme = "light" | "dark" | "hologram";

interface AppSettings {
    hideDonationPrompt: boolean;
    theme: Theme;
}

// Use LazyStore to prevent SSR issues. It will only load when first accessed.
const store = new LazyStore(".settings.dat");

// Create Svelte stores to hold settings in memory for easy access.
// We provide sensible defaults.
export const hideDonationPrompt = writable<boolean>(false);
export const theme = writable<Theme>("light");

/**
 * Loads all settings from the persistent file store and updates the Svelte stores.
 */
export async function loadSettings() {
    const settings = await store.get<AppSettings>("allSettings");
    if (settings) {
        hideDonationPrompt.set(settings.hideDonationPrompt);
        theme.set(settings.theme);
    }
}

/**
 * Saves a new value for a specific setting to the persistent store.
 * @param key The setting key to update.
 * @param value The new value for the setting.
 */
async function saveSetting<T extends keyof AppSettings>(
    key: T,
    value: AppSettings[T],
) {
    // Get the current settings, update the specific key, and save it back.
    const currentSettings = (await store.get<AppSettings>("allSettings")) || {
        hideDonationPrompt: false,
        theme: "light",
    };

    currentSettings[key] = value;
    await store.set("allSettings", currentSettings);
    await store.save();
}

/**
 * Sets the 'hideDonationPrompt' setting to true and saves it.
 */
export async function setHideDonationPrompt() {
    hideDonationPrompt.set(true);
    await saveSetting("hideDonationPrompt", true);
}

/**
 * Sets the application theme and saves it.
 * @param newTheme The new theme to apply.
 */
export async function setTheme(newTheme: Theme) {
    theme.set(newTheme);
    await saveSetting("theme", newTheme);
}
