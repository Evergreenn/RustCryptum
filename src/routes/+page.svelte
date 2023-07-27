<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
	import { AppShell } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { Toast } from '@skeletonlabs/skeleton';
	import { Modal } from '@skeletonlabs/skeleton';
	// import { invoke } from '@tauri-apps/api/tauri';
	import ImportDatabase from '$lib/Forms/ImportDatabase.svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	// your script goes here
	let init: boolean = false;
	let currentTile: number = 0;
	let databasePromise: Promise<any>;

	const getDatabase = async () => {
		const res = await invoke('get_keys');
		console.log('database: ', res);
		return res;
	};

	const toggleInit = () => {
		init = !init;
	};

	databasePromise = getDatabase();
</script>

<Modal />
<Toast />

<AppShell>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="1.5"
					stroke="currentColor"
					class="w-6 h-6"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
					/>
				</svg>

				Rustylock</svelte:fragment
			>
			<svelte:fragment slot="trail">
				<LightSwitch />
			</svelte:fragment>
			<!-- <svelte:fragment slot="headline">(headline)</svelte:fragment> -->
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
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
				<!-- {@debug database} -->
				<!-- <div class="grid place-content-center place-items-center"> -->
				<SidebarLeft bind:currentTile groups={database.groups} />
			{/await}
		{/if}
	</svelte:fragment>
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
			<!-- {@debug database} -->
			<!-- <div class="grid place-content-center place-items-center"> -->
			{#if currentTile === 0}
				<MainLayout entries={database.entries} />
			{:else}
				<MainLayout entries={database.groups[currentTile - 1].entries} />
			{/if}
		{/await}
	{:else}
		<!-- <slot /> -->
		<div class="flex items-center justify-center h-screen p-4">
			<ImportDatabase {toggleInit} />
		</div>
	{/if}
	<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
</AppShell>
