<script lang="ts">
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import TopBar from '$lib/TopBar/TopBar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { useStateStore } from '$lib/stores/stateStore';
	import { goto } from '$app/navigation';
	import { setContext } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import Loader from '$lib/Ui/Loader.svelte';

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

	$: listen('refresh_ui', (_) => {
		databasePromise = getDatabase();
	});
</script>

{#if initialized === true}
	{#await databasePromise}
		<Loader />
	{:then database}
		<div class="flex flex-row w-full">
			<SidebarLeft bind:currentTile groups={database.groups} />
			<div class="flex flex-col basis-4/5 top-0 mt-1 pl-2 overflow-auto h-screen">
				<TopBar />
				<div
					class="overflow-auto pb-10
          pr-2"
				>
					<slot />
				</div>
			</div>
		</div>
	{/await}
{/if}
<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
