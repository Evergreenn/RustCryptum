<script lang="ts">
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import { Toast } from '@skeletonlabs/skeleton';
	import { Modal } from '@skeletonlabs/skeleton';
	import TopBar from '$lib/TopBar/TopBar.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { useStateStore } from '$lib/stores/stateStore';
	import { goto } from '$app/navigation';
	import { setContext } from 'svelte';
	import ImportDatabase from '$lib/Forms/ImportDatabase.svelte';
	const state = useStateStore();
	const { initialized } = $state;
	let currentTile: number = 1;
	let databasePromise: Promise<any>;

	const getDatabase = async () => {
		const res = invoke('get_keys');
		// res.then(
		// 	(res) => {
		// 		setContext('database', res);
		// 		console.log(res);
		// 	},
		// 	(err) => {
		// 		console.log(err);
		// 	}
		// );
		// console.log('database: ', res);
		return res;
	};

	$: {
		if (initialized !== true) {
			goto('/');
		}
	}

	databasePromise = getDatabase();

	$: {
		console.log('currentTile: ', currentTile);
		setContext('currentTile', currentTile);
	}
	$: setContext('database', databasePromise);
</script>

<Modal />
<Toast />

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
			<!-- {@debug database} -->
			<!-- <div class="grid place-content-center place-items-center"> -->
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
