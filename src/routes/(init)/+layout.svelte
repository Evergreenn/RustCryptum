<script lang="ts">
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import TopBar from '$lib/TopBar/TopBar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { useStateStore } from '$lib/stores/stateStore';
	import { goto } from '$app/navigation';
	import { setContext } from 'svelte';
	import { emit, listen } from '@tauri-apps/api/event';

	const state = useStateStore();
	const { initialized } = $state;
	let currentTile: number = 1;
	let databasePromise: Promise<any>;

	let t = useStateStore();

	const getDatabase = async () => {
		const res = invoke('get_keys');
		return res;
	};

	databasePromise = getDatabase();

	$: {
		if (initialized !== true) {
			goto('/');
		}
		setContext('currentTile', ($t.currentTile = currentTile));
		setContext('database', databasePromise);
	}

	const event = listen('refresh_ui', (event) => {
		databasePromise = getDatabase();
	});
</script>

{#if initialized === true}
	{#await databasePromise}
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
		<div class="flex flex-row w-full">
			<SidebarLeft
				bind:currentTile
				groups={database.groups}
				databaseName={database.meta.database_name}
			/>
			<div class="flex flex-col basis-4/5 top-0 pl-2 overflow-auto h-screen">
				<TopBar />
				<slot />
			</div>
		</div>
	{/await}
{/if}
<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
