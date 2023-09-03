<script lang="ts">
	import { page } from '$app/stores';
	import Logo from '$lib/Ui/Logo/Logo.svelte';
	import NodeTreeViewChild from '$lib/Ui/NodeTreeViewChild.svelte';
	import UlViewChild from '$lib/Ui/UlViewChild.svelte';
	import { TreeView, TreeViewItem } from '@skeletonlabs/skeleton';

	export let currentTile: number = 0;
	export let groups: [] = [];
	export let databaseName: string = '';

	let myTreeView: TreeView;
	$: classesActive = (href: string) => (href === $page.url.pathname ? '!bg-primary-500' : '');

	const toggleTile = (e, uuid: number) => {
		e.preventDefault();
		currentTile = uuid;
		return;
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
	<div class=" flex flex-col items-center justify-between">
		<!-- <div class="max-w-3xl"> -->
		<Logo />
	</div>
	<hr class="!border-t-4 ml-6 mr-6 mb-4" />

	<!-- <TreeView on:click={() => myTreeView.expandAll()} bind:this={myTreeView}> -->
	<!-- 	<NodeTreeViewChild children={tree} handleTileClick={toggleTile} /> -->
	<!-- </TreeView> -->

	<nav class="list-nav">
		<!-- (optionally you can provide a label here) -->
		<ul>
			<UlViewChild children={tree} handleTileClick={toggleTile} />
		</ul>
	</nav>
</div>
