<script lang="ts">
	import { currentView, fileViewMode, rightSidebar } from '$lib/stores';
	import { tags } from '$lib/worldStore';
	import TagIndexView from '$lib/components/TagIndexView.svelte';
	import FileView from '$lib/components/FileView.svelte';
	import BacklinksPanel from '$lib/components/BacklinksPanel.svelte';
	import { getTitleFromPath } from '$lib/utils';

	// This derived state automatically recalculates when the view or tags change,
	// ensuring the tag view is always up-to-date without a manual $effect.
	const tagViewData = $derived.by(() => {
		const view = $currentView;
		if (view.type !== 'tag') return null;

		const latestTagData = $tags.find(([name]) => name === view.data.name);

		// If the tag is no longer found (e.g., all pages with it were deleted),
		// we can handle it gracefully. Here we return null, and the template will
		// simply not render the TagIndexView.
		if (!latestTagData) {
			return null;
		}

		// If the tag is found, create the data object for the view.
		return {
			name: latestTagData[0],
			pages: latestTagData[1].map((path) => ({
				path,
				title: getTitleFromPath(path)
			}))
		};
	});

	// This effect resets the file view mode and hides the right sidebar
	// whenever the user navigates away from the file view.
	$effect(() => {
		if ($currentView.type !== 'file') {
			$fileViewMode = 'preview';
			rightSidebar.update((state) => ({ ...state, isVisible: false }));
		}
	});
</script>

{#if $currentView.type === 'welcome'}
	<div class="welcome-screen">
		<img src="/compass.svg" alt="Compass" class="welcome-icon" />
		<h1 class="welcome-title">Chronicler</h1>
		<p class="welcome-text">Select a page from the sidebar to begin your journey.</p>
	</div>
{:else if $currentView.type === 'tag' && tagViewData}
	<div class="tag-view-pane">
		<TagIndexView data={tagViewData} />
	</div>
{:else if $currentView.type === 'file' && $currentView.data}
	<FileView file={$currentView.data} />
{/if}

{#if $rightSidebar.isVisible}
	<BacklinksPanel />
{/if}

<style>
	.tag-view-pane {
		flex-basis: 100%;
		padding: 2rem;
		height: 100%;
		box-sizing: border-box;
	}
	.welcome-screen {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		width: 100%;
	}
	.welcome-icon {
		width: 150px;
		height: 150px;
		opacity: 0.6;
		margin-bottom: 2rem;
	}
	.welcome-title {
		font-family: 'Uncial Antiqua', cursive;
		font-size: 4rem;
		margin-bottom: 1rem;
		color: #6a5f55;
	}
	.welcome-text {
		font-size: 1.2rem;
	}
</style>
