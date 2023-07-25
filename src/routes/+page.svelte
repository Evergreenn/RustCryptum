<script lang="ts">
	import { AppBar } from '@skeletonlabs/skeleton';
	import { AppShell } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import SidebarLeft from '$lib/sidebar/SidebarLeft.svelte';
	import MainLayout from '$lib/mainLayout/MainLayout.svelte';
	import { Modal } from '@skeletonlabs/skeleton';
	// import { invoke } from '@tauri-apps/api/tauri';
	import ImportDatabase from '$lib/Forms/ImportDatabase.svelte';
	// your script goes here
	let init: boolean = false;
</script>

<Modal />

<AppShell>
	<svelte:fragment slot="header">
		<AppBar>
			<svelte:fragment slot="lead">(icon) Rustylock</svelte:fragment>
			<svelte:fragment slot="trail">
				<LightSwitch />
			</svelte:fragment>
			<!-- <svelte:fragment slot="headline">(headline)</svelte:fragment> -->
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		{#if init === true}
			<SidebarLeft />
		{/if}
	</svelte:fragment>

	{#if init === true}
		<slot />
		<MainLayout />
	{:else}
		<!-- <slot /> -->
		<div class="flex items-center justify-center h-screen p-4">
			<ImportDatabase />
		</div>
	{/if}
	<!-- <svelte:fragment slot="pageFooter">Page Footer</svelte:fragment> -->
</AppShell>
