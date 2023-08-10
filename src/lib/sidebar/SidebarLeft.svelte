<!-- <svelte:options accessors={true} /> -->

<script lang="ts">
	import { page } from '$app/stores';
	import NodeTreeViewChild from '$lib/Ui/NodeTreeViewChild.svelte';
	import { TreeView, TreeViewItem } from '@skeletonlabs/skeleton';
	import { setContext } from 'svelte';
	import logo from '$lib/assets/logo-no-background.svg';

	export let currentTile: number = 0;
	export let groups: [] = [];
	export let databaseName: string = '';

	// $: console.log('groups: ', groups);
	$: classesActive = (href: string) => (href === $page.url.pathname ? '!bg-primary-500' : '');

	const toggleTile = (e, uuid: number) => {
		e.preventDefault();
		// if (e.detail.open) {
		console.log('event: ', e);
		currentTile = uuid;
		return;
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
</script>

<div class="bg-suface-200-700-token h-screen basis-1/5">
	<!-- <div class=" flex flex-row items-center justify-between mb-4"> -->
	<div class="max-w-3xl">
		<img src={logo} class="h-auto max-w-full mt-5 mb-6" />
	</div>
	<hr class="!border-t-4 ml-6 mr-6 mb-4" />

	<TreeView>
		<NodeTreeViewChild children={tree} handleTileClick={toggleTile} />
	</TreeView>
</div>
