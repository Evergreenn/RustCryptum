<script lang="ts">
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { Toast } from '@skeletonlabs/skeleton';
	import { Modal } from '@skeletonlabs/skeleton';
	import TopBar from '$lib/TopBar/TopBar.svelte';
	// import { invoke } from '@tauri-apps/api/tauri';
	import ImportDatabase from '$lib/Forms/ImportDatabase.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { writable } from 'svelte/store';
	// your script goes here
	let init: boolean = false;
	let currentTile: number = 1;
	let databasePromise: Promise<any>;

	const getDatabase = async () => {
		const res = await invoke('get_keys');
		console.log('database: ', res);
		return res;
	};

	// const toggleInit = () => {
	// init = !init;
	$: init = databasePromise = getDatabase();
	// };

	$: console.log('currentTile: ', currentTile);

	// const handleTileClick = (uuid: number) => {
	// 	console.log('handleTileClick: ', uuid);
	// 	currentTile = uuid;
	// };
</script>

<Modal />
<Toast />

{#if init === true}
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
			<div class="flex flex-col h-36 basis-4/5 top-0 pl-2 overflow-auto h-screen">
				<TopBar />
				<slot />
			</div>
		</div>
		<!-- {#if currentTile === 0} -->
		<!-- <MainLayout entries={database} /> -->
		<!-- {:else} -->
		<!-- {/if} -->
	{/await}
{:else}
	<!-- <slot /> -->
	<div class="flex items-center justify-center h-screen p-4">
		<ImportDatabase {toggleInit} />
	</div>
{/if}
<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
