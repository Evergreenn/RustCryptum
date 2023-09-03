<script lang="ts">
	import { page } from '$app/stores';
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { useStateStore } from '$lib/stores/stateStore';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { getContext } from 'svelte';

	// let statestore = useStateStore();
	let currentTile: string = '';

	let database: any = [];
	$: {
		// currentTile = $statestore.currentTile;
		currentTile = $page.params.slug;
		database = getContext('database');
	}
</script>

{#await database}
	<div class="flex flex-col items-center justify-center h-screen">
		<ProgressRadial
			...
			stroke={20}
			meter="stroke-primary-500"
			track="stroke-primary-500/30"
			value={undefined}
		/>
	</div>
{:then database}
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
		<MainLayout {database} {currentTile} />
	</div>
{/await}
<!-- {#each database as tile (tile.uuid)} -->
<!-- 	{#if tile.uuid === currentTile} -->
<!-- 		<MainLayout {database} {currentTile} /> -->
<!-- 	{/if} -->
<!-- {/each} -->
<!-- <MainLayout {database} {currentTile} /> -->
