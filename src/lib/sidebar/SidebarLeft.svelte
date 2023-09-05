<script lang="ts">
	import Loader from '$lib/Ui/Loader.svelte';
	import Logo from '$lib/Ui/Logo/Logo.svelte';
	import UlViewChild from '$lib/Ui/UlViewChild.svelte';

	export let currentTile: number = 0;
	export let groups: [] = [];
	// let tree: [] = [];

	const toggleTile = (e: Event, uuid: number) => {
		e.preventDefault();
		currentTile = uuid;
		return;
	};

	// const buildGroupTree = (groups: any) => {
	// 	let tree: any = [];
	// 	if (groups.length > 0) {
	// 		groups.forEach((group: any) => {
	// 			tree.push({
	// 				uuid: group.uuid,
	// 				name: group.name,
	// 				groups: buildGroupTree(group.groups)
	// 			});
	// 		});
	// 	}
	// 	return tree;
	// };

	// $: tree = buildGroupTree(groups);
	// $: tree = groups;
</script>

<div class="bg-suface-200-700-token h-screen basis-1/5">
	<div class=" flex flex-col items-center justify-between">
		<Logo />
	</div>
	<hr class="!border-t-4 ml-6 mr-6 mb-4" />

	{#if !groups || groups.length === 0}
		<Loader title="Loading the database" />
	{:else}
		<nav class="list-nav">
			<ul>
				<UlViewChild children={groups} handleTileClick={toggleTile} />
			</ul>
		</nav>
	{/if}
</div>
