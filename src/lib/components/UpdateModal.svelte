<script lang="ts">
    import { onMount } from "svelte";
    import type { Update } from "@tauri-apps/plugin-updater";
    import { getVersion } from "@tauri-apps/api/app";
    import { readTextFile } from "@tauri-apps/plugin-fs";
    import { resolveResource } from "@tauri-apps/api/path";
    import { installUpdate, openReleasePage } from "$lib/updater";
    import Modal from "$lib/components/Modal.svelte";

    let { update, manualUpdateRequired, onClose } = $props<{
        update: Update;
        manualUpdateRequired: boolean;
        onClose: () => void;
    }>();

    let isUpdating = $state(false);
    let installError = $state<string | null>(null);
    let currentVersion = $state<string | null>(null);
    let changelogContent = $state<string | null>(null);

    /**
     * Takes the raw markdown from CHANGELOG.md and formats it to show only
     * the relevant new entries in a clean, concise list.
     */
    function formatChangelog(
        rawText: string | null,
        version: string | null,
    ): string | null {
        if (!rawText || !version) return null;

        // 1. Find the user's current version in the log.
        const versionHeader = `## [v${version}`;
        const versionIndex = rawText.indexOf(versionHeader);

        // 2. Slice the text to get only the content *before* the user's version.
        const relevantText =
            versionIndex !== -1 ? rawText.substring(0, versionIndex) : rawText;

        // 3. Find the start of the actual content, skipping the main header.
        const contentStartIndex = relevantText.indexOf("---");
        if (contentStartIndex === -1) return relevantText;

        const content = relevantText.substring(contentStartIndex);

        // 4. Process each line to reformat it.
        return content
            .split("\n")
            .map((line) => {
                const trimmedLine = line.trim();
                // Return null for blank lines to filter them out later.
                if (trimmedLine === "") return null;
                // Remove separators
                if (trimmedLine === "---") return null;
                // Keep version headers, but remove the link part for cleanliness
                if (trimmedLine.startsWith("## [")) {
                    return "\n" + trimmedLine.split("]")[0] + "]";
                }
                // Replace markdown list items with a '+'
                if (trimmedLine.startsWith("- ")) {
                    return "+ " + trimmedLine.substring(2);
                }
                // Remove category headers (e.g., ### ✨ Added)
                if (trimmedLine.startsWith("###")) {
                    return null;
                }
                // Ignore other lines
                return null;
            })
            .filter((line) => line !== null) // Remove the null (blank/ignored) lines
            .join("\n")
            .trim();
    }

    const formattedChangelog = $derived(
        formatChangelog(changelogContent, currentVersion),
    );

    // Fetch the current app version and the changelog when the component is mounted
    onMount(async () => {
        try {
            currentVersion = await getVersion();
            const resourcePath = await resolveResource("CHANGELOG.md");
            changelogContent = await readTextFile(resourcePath);
        } catch (e) {
            console.error("Failed to get app info or changelog:", e);
            changelogContent = "Could not load release notes.";
        }
    });

    async function handleInstallClick() {
        isUpdating = true;
        installError = null;
        try {
            await installUpdate(update);
            // On success, the app will relaunch, so no need to set isUpdating = false.
        } catch (error) {
            console.error("Failed to install update:", error);
            installError =
                "Update failed. Please try again or visit the downloads page to update manually.";
            isUpdating = false; // Only reset on error
        }
    }
</script>

<Modal title="Update Available!" {onClose}>
    <p>
        A new version of Chronicler is available: <strong
            >{update.version}</strong
        >
        {#if currentVersion}(you have {currentVersion}){/if}.
    </p>

    {#if formattedChangelog}
        <div class="release-notes">
            <div class="notes-content">{formattedChangelog}</div>
        </div>
    {/if}

    {#if manualUpdateRequired}
        <div class="manual-update-notice">
            <p><strong>Manual Update Required</strong></p>
            <p class="text-sm">
                Since you installed via a system package manager (.deb or .rpm),
                please download the latest version from our releases page.
            </p>
        </div>
        <div class="button-group">
            <button class="button-secondary" onclick={onClose}>Later</button>
            <button class="button-primary" onclick={openReleasePage}
                >Go to Downloads</button
            >
        </div>
    {:else}
        {#if installError}
            <div class="manual-update-notice">
                <p><strong>Update Failed</strong></p>
                <p class="text-sm">{installError}</p>
            </div>
        {/if}
        <div class="button-group">
            <button
                class="button-secondary"
                onclick={onClose}
                disabled={isUpdating}>Later</button
            >
            <button
                class="button-primary"
                onclick={handleInstallClick}
                disabled={isUpdating}
            >
                {#if isUpdating}
                    <span>Updating...</span>
                {:else}
                    <span>Install & Relaunch</span>
                {/if}
            </button>
        </div>
    {/if}
</Modal>

<style>
    .release-notes {
        padding: 1rem;
        background-color: var(--parchment-mid);
        border-radius: 6px;
        max-height: 250px;
        /* Enable both vertical and horizontal scrolling */
        overflow: auto;
        border: 1px solid var(--border-color);
    }
    .notes-content {
        font-size: 0.8rem;
        line-height: 1.7;
        white-space: pre;
        font-family: var(--font-mono);
    }
    .manual-update-notice {
        background-color: var(--parchment-dark);
        border: 1px solid var(--parchment-dark);
        padding: 1rem;
        border-radius: 6px;
        margin-top: 1rem;
        margin-bottom: 1rem;
    }
    .manual-update-notice .text-sm {
        font-size: 0.9rem;
        opacity: 0.9;
    }
    .button-group {
        display: flex;
        justify-content: flex-end;
        gap: 0.75rem;
        margin-top: 1.5rem;
    }
</style>
