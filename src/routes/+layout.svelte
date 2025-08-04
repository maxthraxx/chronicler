<script lang="ts">
    import { onMount } from "svelte";
    import { get } from "svelte/store";
    import {
        SIDEBAR_INITIAL_WIDTH,
        SIDEBAR_MIN_WIDTH,
        SIDEBAR_MAX_WIDTH,
        SIDEBAR_KEYBOARD_RESIZE_STEP,
    } from "$lib/config";
    import { appStatus } from "$lib/appState";
    import {
        initializeApp,
        selectNewVault,
        handleVaultSelected,
    } from "$lib/startup";
    import {
        hideDonationPrompt,
        activeTheme,
        fontSize,
        userThemes,
    } from "$lib/settingsStore";
    import { openModal } from "$lib/modalStore";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    // Import UI Components
    import VaultSelector from "$lib/components/VaultSelector.svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import ModalManager from "$lib/components/ModalManager.svelte";
    import ErrorBox from "$lib/components/ErrorBox.svelte";
    import Button from "$lib/components/Button.svelte";
    import DonationModal from "$lib/components/DonationModal.svelte";

    import "../app.css";

    let { children } = $props();
    let sidebarWidth = $state(SIDEBAR_INITIAL_WIDTH);
    let isResizing = $state(false);

    onMount(() => {
        // Kick off the main application startup sequence
        initializeApp();

        // This UI-specific logic remains here as it's tied to the window lifecycle
        (async () => {
            // Only attach the listener if the user has not opted out.
            if (!get(hideDonationPrompt)) {
                let hasFiredOnce = false;
                const unlisten = await getCurrentWindow().onCloseRequested(
                    async (event) => {
                        if (hasFiredOnce) return;
                        event.preventDefault();
                        hasFiredOnce = true;
                        openModal({ component: DonationModal, props: {} });
                    },
                );
                return () => unlisten();
            }
        })();
    });

    // This effect handles applying the correct theme styles.
    $effect(() => {
        const themeName = $activeTheme;
        const customTheme = $userThemes.find((t) => t.name === themeName);
        if (typeof document !== "undefined") {
            // Always clear any inline styles first.
            document.documentElement.style.cssText = "";
            if (customTheme) {
                // If it's a custom theme, apply its variables as inline styles.
                for (const [key, value] of Object.entries(
                    customTheme.palette,
                )) {
                    document.documentElement.style.setProperty(key, value);
                }
                // Also remove the data-theme attribute so it doesn't conflict.
                document.documentElement.removeAttribute("data-theme");
            } else {
                // If it's a built-in theme, use the data-theme attribute.
                document.documentElement.setAttribute(
                    "data-theme",
                    themeName || "light",
                );
            }
        }
    });

    // This effect applies the font size.
    $effect(() => {
        if (typeof document !== "undefined") {
            document.documentElement.style.fontSize = `${$fontSize}%`;
        }
    });

    /** Resets the application state to allow the user to select a new vault. */
    function handleTryAgain() {
        selectNewVault();
    }

    // --- Sidebar Resizing Logic ---

    /** Initiates the sidebar resizing drag operation. */
    function startResize() {
        isResizing = true;
        // Add passive option for better scroll performance during resize.
        window.addEventListener("mousemove", doResize, { passive: true });
        window.addEventListener("mouseup", stopResize); // Ensure it only fires once
    }

    /** Performs the resize operation based on mouse movement. */
    function doResize(event: MouseEvent) {
        if (isResizing) {
            const newWidth = event.clientX;
            if (newWidth > SIDEBAR_MIN_WIDTH && newWidth < SIDEBAR_MAX_WIDTH) {
                sidebarWidth = newWidth;
            }
        }
    }

    /** Stops the resizing drag operation and cleans up event listeners. */
    function stopResize() {
        isResizing = false;
        window.removeEventListener("mousemove", doResize);
        window.removeEventListener("mouseup", stopResize);
    }

    /** Handles resizing the sidebar using the keyboard for accessibility. */
    function handleKeyResize(event: KeyboardEvent) {
        if (event.key === "ArrowLeft") {
            event.preventDefault();
            const newWidth = Math.max(
                SIDEBAR_MIN_WIDTH,
                sidebarWidth - SIDEBAR_KEYBOARD_RESIZE_STEP,
            );
            sidebarWidth = newWidth;
        } else if (event.key === "ArrowRight") {
            event.preventDefault();
            const newWidth = Math.min(
                SIDEBAR_MAX_WIDTH,
                sidebarWidth + SIDEBAR_KEYBOARD_RESIZE_STEP,
            );
            sidebarWidth = newWidth;
        }
    }
</script>

<ModalManager />

{#if $appStatus.state === "selecting_vault"}
    <VaultSelector onVaultSelected={handleVaultSelected} />
{:else if $appStatus.state === "loading"}
    <div class="loading-screen">
        <img
            src="/compass.png"
            alt="Compass"
            class="welcome-icon animate-spin"
        />
        <h1 class="welcome-title">Opening Vault...</h1>
    </div>
{:else if $appStatus.state === "error"}
    <div class="loading-screen">
        <h1 class="welcome-title">Error</h1>
        <ErrorBox>{$appStatus.message}</ErrorBox>
        <Button onclick={handleTryAgain}>Select a Different Folder</Button>
    </div>
{:else if $appStatus.state === "ready"}
    <div class="chronicler-app" style="--sidebar-width: {sidebarWidth}px">
        <Sidebar bind:width={sidebarWidth} />

        <div
            class="resizer"
            onmousedown={startResize}
            onkeydown={handleKeyResize}
            role="slider"
            tabindex="0"
            aria-label="Resize sidebar"
            aria-orientation="vertical"
            aria-valuenow={sidebarWidth}
            aria-valuemin={SIDEBAR_MIN_WIDTH}
            aria-valuemax={SIDEBAR_MAX_WIDTH}
            style="left: {sidebarWidth - 2.5}px;"
        ></div>

        <main class="main-content">
            {@render children()}
        </main>
    </div>
{/if}

<style>
    .loading-screen {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        width: 100vw;
        height: 100vh;
        color: var(--color-text-primary);
        padding: 2rem;
    }
    .chronicler-app {
        display: flex;
        height: 100vh;
        width: 100vw;
        color: var(--color-text-primary);
        font-family: var(--font-family-body);
    }
    .main-content {
        display: flex;
        flex-grow: 1;
        height: 100%;
        margin-left: var(--sidebar-width);
    }
    .resizer {
        width: 5px;
        cursor: ew-resize;
        background: var(--color-overlay-resizer);
        position: fixed;
        top: 0;
        bottom: 0;
        z-index: 100;
        transition: background-color 0.2s;
    }
    .resizer:hover,
    .resizer:focus {
        background: var(--color-overlay-resizer-hover);
        outline: none;
    }
    .welcome-icon {
        width: 150px;
        height: 150px;
        opacity: 0.8;
        margin-bottom: 2rem;
    }
    .welcome-title {
        font-family: var(--font-family-heading);
        font-size: 4rem;
        margin-bottom: 1rem;
        color: var(--color-text-heading);
    }
    .animate-spin {
        animation: spin 2s linear infinite;
    }
    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>
