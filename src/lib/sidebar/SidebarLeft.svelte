<svelte:options accessors={true} />

<script lang="ts">
	// import { invoke } from '@tauri-apps/api/tauri';
	import { page } from '$app/stores';
	import NodeTreeViewChild from '$lib/Ui/NodeTreeViewChild.svelte';
	import { TreeView, TreeViewItem } from '@skeletonlabs/skeleton';

	export let currentTile: number = 0;
	export let groups: [] = [];
	export let databaseName: string = '';

	$: console.log('groups: ', groups);
	$: classesActive = (href: string) => (href === $page.url.pathname ? '!bg-primary-500' : '');

	const toggleTile = (e, uuid: number) => {
		console.log(e.detail.open);
		// if (e.detail.open) {
			currentTile = uuid;
		// }
    // else {
			// currentTile = 0;
		// }
	};

	const buildGroupTree = (groups: any) => {
		let tree: any = [];
		if (groups.length > 0) {
			groups.forEach((group: any) => {
				tree.push({
					uuid: group.uuid,
					name: group.name,
					groups: buildGroupTree(group.groups)
				});
			});
		}
		return tree;
	};

	const tree = buildGroupTree(groups);
	$: console.log('tree: ', tree);
</script>

<div class="bg-suface-100-800-token variant-glass-surface h-screen basis-1/5">
	<div class=" flex flex-row items-center justify-between mb-4">
		<div class="flex items-center justify-between w-1/4 ml-3 mt-4">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="w-20 h-20"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"
				/>
			</svg>
		</div>
		<div class="flex flex-col justify-center w-3/4 items-center">
			<h1 class="text-3xl">Rustylock</h1>
		</div>
	</div>
	<hr class="!border-t-4 ml-6 mr-6 mb-4" />

	<TreeView>
		<NodeTreeViewChild children={tree} handleTileClick={toggleTile} />
	</TreeView>
</div>
