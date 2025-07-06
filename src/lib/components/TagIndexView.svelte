<script lang="ts">
    import { currentView } from "$lib/stores";
    import type { PageHeader } from "$lib/bindings";
    import type { TagIndexData } from "$lib/stores";

    let { data } = $props<{ data: TagIndexData }>();

    // Clicking a file link in this view switches back to the file view.
    function openFile(file: PageHeader) {
        currentView.set({ type: "file", data: file });
    }
</script>

<div class="tag-index-wrapper">
    <h2>Index for <span class="tag-highlight">#{data.name}</span></h2>

    <ul class="page-link-list">
        {#each data.pages as page (page.path)}
            <li>
                <button class="link-button" onclick={() => openFile(page)}>
                    {page.title}
                </button>
            </li>
        {/each}
    </ul>
</div>

<style>
    .tag-index-wrapper {
        padding: 2rem;
        height: 100%;
        overflow-y: auto;
        box-sizing: border-box;
    }
    h2 {
        font-family: "Uncial Antiqua", cursive;
        color: var(--ink-light);
        border-bottom: 1px solid var(--border-color);
        padding-bottom: 0.5rem;
        margin-top: 0;
        margin-bottom: 1rem;
    }
    .tag-highlight {
        color: var(--ink);
        font-weight: bold;
    }
    .page-link-list {
        list-style: disc;
        padding-left: 2rem;
    }

    .page-link-list li {
        margin-bottom: 0.5rem;
    }

    /* Target the global helper class within this component */
    .page-link-list :global(.link-button) {
        font-size: 1.1rem;
    }
</style>
