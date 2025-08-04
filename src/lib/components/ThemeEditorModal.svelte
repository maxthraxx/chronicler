<script lang="ts">
    import Modal from "./Modal.svelte";
    import Button from "./Button.svelte";
    import {
        userThemes,
        saveCustomTheme,
        deleteCustomTheme,
        type CustomTheme,
        type ThemePalette,
    } from "$lib/settingsStore";

    let { onClose } = $props<{ onClose: () => void }>();

    // The theme currently being edited in the form.
    let currentTheme: CustomTheme | null = $state(null);
    let originalName: string | null = $state(null);

    // A map of variable names to user-friendly labels for the form.
    const colorLabels: Record<keyof ThemePalette, string> = {
        "--color-background-primary": "Primary Background",
        "--color-background-secondary": "Secondary Background",
        "--color-background-tertiary": "Tertiary Background",
        "--color-text-heading": "Header Text",
        "--color-text-primary": "Primary Text",
        "--color-text-secondary": "Secondary Text",
        "--color-border-primary": "Borders",
        "--color-accent-primary": "Accent",
        "--color-text-link": "Links",
        "--color-text-link-broken": "Broken Links",
        "--color-text-error": "Errors",
    };

    const defaultPalette: ThemePalette = {
        "--color-background-primary": "#fdf6e3",
        "--color-background-secondary": "#e6dcc9",
        "--color-background-tertiary": "#dcd3c3",
        "--color-text-heading": "#6a5f55",
        "--color-text-primary": "#4a3f35",
        "--color-text-secondary": "#6a5f55",
        "--color-border-primary": "#d3c7b3",
        "--color-accent-primary": "#dcd3c3",
        "--color-text-link": "#2563eb",
        "--color-text-link-broken": "#b04a4a",
        "--color-text-error": "#8b0000",
    };

    function createNewTheme() {
        currentTheme = {
            name: "My New Theme",
            palette: { ...defaultPalette },
        };
        originalName = null;
    }

    function editTheme(theme: CustomTheme) {
        currentTheme = JSON.parse(JSON.stringify(theme)); // Deep copy
        originalName = theme.name;
    }

    function handleSave(event: SubmitEvent) {
        event.preventDefault();

        const themeToSave = currentTheme;
        if (!themeToSave || !themeToSave.name.trim()) {
            alert("Theme name cannot be empty.");
            return;
        }

        if (
            originalName !== themeToSave.name &&
            $userThemes.some((t) => t.name === themeToSave.name)
        ) {
            alert("A theme with this name already exists.");
            return;
        }

        if (originalName && originalName !== themeToSave.name) {
            deleteCustomTheme(originalName);
        }

        saveCustomTheme(themeToSave);
        currentTheme = null; // Exit edit mode
    }

    function handleDelete() {
        const themeToDelete = currentTheme;
        if (!themeToDelete) {
            return;
        }

        if (
            window.confirm(
                `Are you sure you want to delete "${themeToDelete.name}"?`,
            )
        ) {
            deleteCustomTheme(themeToDelete.name);
            currentTheme = null; // Exit edit mode
        }
    }
</script>

<Modal title="Theme Editor" {onClose}>
    <div class="theme-editor-container">
        <aside class="theme-list-panel">
            <h4>Custom Themes</h4>
            <ul class="theme-list">
                {#each $userThemes as theme (theme.name)}
                    <li>
                        <button
                            class="theme-item"
                            onclick={() => editTheme(theme)}
                        >
                            {theme.name}
                        </button>
                    </li>
                {/each}
            </ul>
            <Button onclick={createNewTheme}>+ Create New Theme</Button>
        </aside>

        <main class="theme-form-panel">
            {#if currentTheme}
                <form onsubmit={handleSave}>
                    <div class="form-group">
                        <label for="theme-name">Theme Name</label>
                        <input
                            id="theme-name"
                            type="text"
                            bind:value={currentTheme.name}
                        />
                    </div>

                    <h4>Colors</h4>
                    <div class="color-grid">
                        {#each Object.keys(colorLabels) as key (key)}
                            {@const paletteKey = key as keyof ThemePalette}
                            <div class="form-group color-picker-group">
                                <label for="color-{paletteKey}"
                                    >{colorLabels[paletteKey]}</label
                                >
                                <div class="color-input-wrapper">
                                    <input
                                        id="color-{paletteKey}"
                                        type="color"
                                        bind:value={
                                            currentTheme.palette[paletteKey]
                                        }
                                    />
                                    <span
                                        >{currentTheme.palette[
                                            paletteKey
                                        ]}</span
                                    >
                                </div>
                            </div>
                        {/each}
                    </div>

                    <div class="form-actions">
                        <Button type="submit">Save Theme</Button>
                        {#if originalName}
                            <Button
                                variant="ghost"
                                type="button"
                                onclick={handleDelete}
                            >
                                Delete
                            </Button>
                        {/if}
                        <Button
                            variant="ghost"
                            type="button"
                            onclick={() => (currentTheme = null)}
                        >
                            Cancel
                        </Button>
                    </div>
                </form>
            {:else}
                <div class="placeholder">
                    <p>Select a theme to edit or create a new one.</p>
                </div>
            {/if}
        </main>
    </div>
</Modal>

<style>
    .theme-editor-container {
        display: flex;
        gap: 2rem;
        /* Set a max-height relative to the viewport and a min-height */
        max-height: 70vh;
        min-height: 450px;
    }
    .theme-list-panel {
        flex: 0 0 200px;
        border-right: 1px solid var(--color-border-primary);
        padding-right: 2rem;
    }
    .theme-list {
        list-style: none;
        padding: 0;
        margin: 0 0 1rem 0;
    }
    .theme-item {
        width: 100%;
        background: none;
        border: none;
        color: var(--color-text-link);
        text-align: left;
        padding: 0.5rem;
        border-radius: 4px;
        cursor: pointer;
    }
    .theme-item:hover {
        background-color: var(--color-background-secondary);
    }
    .theme-form-panel {
        flex: 1;
        /* This is the key change: allow this panel to scroll vertically */
        overflow-y: auto;
        /* Add some padding to the right to make space for the scrollbar */
        padding-right: 1rem;
        /* A crucial property for flexbox children to scroll correctly */
        min-height: 0;
    }
    .form-group {
        margin-bottom: 1rem;
    }
    .form-group label {
        display: block;
        margin-bottom: 0.5rem;
        color: var(--color-text-secondary);
    }
    .form-group input[type="text"] {
        width: 100%;
        padding: 0.5rem;
        font-size: 1rem;
        border: 1px solid var(--color-border-primary);
        background-color: var(--color-background-secondary);
        color: var(--color-text-primary);
        border-radius: 4px;
    }
    .color-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
        gap: 1rem;
        /* Add padding to the bottom so the save button doesn't stick */
        padding-bottom: 1rem;
    }
    .color-input-wrapper {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
    .color-input-wrapper input[type="color"] {
        width: 40px;
        height: 40px;
        border: 1px solid var(--color-border-primary);
        padding: 0;
        border-radius: 4px;
        cursor: pointer;
    }
    .form-actions {
        margin-top: 2rem;
        display: flex;
        gap: 0.5rem;
    }
    .placeholder {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 100%;
        color: var(--color-text-secondary);
    }
</style>
