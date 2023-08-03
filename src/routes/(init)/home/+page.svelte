<script lang="ts">
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { useStateStore } from '$lib/stores/stateStore';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { getContext } from 'svelte';

	let statestore = useStateStore();
	let database: any = [];
	let currentTile: number = 0;

	$: {
		currentTile = $statestore.currentTile;
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
	<MainLayout {database} {currentTile} />
{/await}
