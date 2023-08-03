<script lang="ts">
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { getContext } from 'svelte';
	let data;
	let tile;
	let currentTile = getContext('currentTile');
	const database = getContext('database');
	// console.log('database after context: ', database);
	// const currentTile = getContext('currentTile');
	$: data = database;
	$: {
		console.log('currentTile: in child ', currentTile);
		// currentTile = getContext('currentTile');
		tile = getContext('currentTile');
	}
</script>

<!-- {#if database && currentTile} -->
{#await data}
	<div class="flex flex-col items-center justify-center h-screen">
		<ProgressRadial
			...
			stroke={20}
			meter="stroke-primary-500"
			track="stroke-primary-500/30"
			value={undefined}
		/>
	</div>
{:then data}
	<MainLayout database={data} currentTile={tile} />
{/await}
<!-- {/if} -->
