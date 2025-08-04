/**
 * @file This file centralizes the application's startup and initialization logic.
 */

import { resetAllStores } from "$lib/viewStores";
import { appStatus } from "$lib/appState";
import { world } from "$lib/worldStore";
import { initializeVault } from "$lib/actions";
import { getVaultPath } from "$lib/commands";
import { loadSettings } from "$lib/settingsStore";
import { checkForAppUpdates } from "$lib/updater";

/**
 * Orchestrates the complete vault initialization sequence. This function is the
 * primary entry point after a vault path is chosen.
 * @param path The absolute path to the selected vault directory.
 */
async function handleVaultSelected(path: string) {
    appStatus.set({ state: "loading" });
    try {
        // 1. Initialize the backend state
        await initializeVault(path);
        // 2. Initialize the frontend stores
        await world.initialize();
        // 3. Set status to ready ONLY after everything is finished
        appStatus.set({ state: "ready" });

        // After the app is ready, check for updates in the background.
        checkForAppUpdates();
    } catch (e: any) {
        appStatus.set({ state: "error", message: e.message });
    }
}

/**
 * The main application entry point, called on mount from the root layout.
 * It loads settings, finds the last-used vault path, and kicks off initialization.
 */
export async function initializeApp() {
    try {
        await loadSettings();
        const path = await getVaultPath();
        if (path) {
            await handleVaultSelected(path);
        } else {
            appStatus.set({ state: "selecting_vault" });
        }
    } catch (e: any) {
        console.error("Failed during startup initialization:", e);
        const errorMessage = e.message || `Failed to read configuration: ${e}`;
        appStatus.set({ state: "error", message: errorMessage });
    }
}

/**
 * Resets the application state to allow the user to select a new vault.
 * It destroys the current world state, resets all UI stores, and then
 * explicitly sets the application status back to the vault selection screen.
 */
export function selectNewVault() {
    world.destroy();
    resetAllStores();
    appStatus.set({ state: "selecting_vault" });
}

// Re-export handleVaultSelected for use in the VaultSelector component
export { handleVaultSelected };
