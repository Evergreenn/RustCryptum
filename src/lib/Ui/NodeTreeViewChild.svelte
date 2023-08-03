<script lang="ts">
	import { TreeViewItem } from '@skeletonlabs/skeleton';
	import { useStateStore } from '$lib/stores/stateStore';

	export let children: any = [];
	export let handleTileClick: (e: any, uuid: number) => void;
	let statestore = useStateStore();
	let current: string = '';

	const setBreadcrumb = (title: string) => {
		$statestore.breadcrumb = title;
	};

	const open = (_id: number) => {
		// 	if (id === 0) {
		// 		return true;
		// 	}
		return false;
	};

	const onToggle = (e: any, uuid: string) => {
		if (e.detail.open) {
			current = uuid;
		} else {
			current = '';
		}
	};
</script>

{#each children as child, i}
	<TreeViewItem
		regionSummary={current === child.uuid ? 'font-bold' : ''}
		open={open(i)}
		on:toggle={(e) => {
			handleTileClick(e, child.uuid), onToggle(e, child.uuid), setBreadcrumb(child.name);
		}}
	>
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
					d="M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
				/>
			</svg>
		</svelte:fragment>
		{child.name}
		<svelte:fragment slot="children">
			{#if child.groups.length > 0}
				<svelte:self children={child.groups} {handleTileClick} />
			{/if}
		</svelte:fragment>
	</TreeViewItem>
{/each}

<style>
</style>
