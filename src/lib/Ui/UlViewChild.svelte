<script lang="ts">
	import Folder from '$lib/icons/Folder.svelte';
	import { useStateStore } from '$lib/stores/stateStore';

	export let parent_uuid: number = 0;
	export let depth: number = 2;
	export let children: any = [];
	export let handleTileClick: (e: any, uuid: number) => void;

	let statestore = useStateStore();
	let toString: string = '';

	const setBreadcrumb = (title: string, uuid: string) => {
		$statestore.breadcrumb = title;
		$statestore.currentFolderUuid = uuid;
	};

	$: classesActive = (uuid: string) => {
		return uuid === $statestore.currentFolderUuid ? '!underline underline-offset-8' : '';
	};

	$: {
		if (parent_uuid) {
			depth = depth + 10;
		} else {
			depth = 2;
		}
		//HACK: this is a hack to get the margin-left to work
		toString = `margin-left: ${depth}px`;
		console.log('toString', toString);
	}

	// $: children;
</script>

{#each children as child}
	<!-- <li class=" ml-[22px] {classesActive(child.uuid)}"> -->
	<!-- <li class=" ml-[382px] {classesActive(child.uuid)}"> -->
	<li style={toString} class=" {classesActive(child.uuid)}">
		<a
			href="/main/{child.uuid}"
			on:click={() => {
				setBreadcrumb(child.name, child.uuid);
			}}
		>
			<span class="badge bg-primary-500">
				<Folder />
			</span>
			<span class="flex-auto">{child.name}</span>
		</a>
	</li>
	{#if child.groups.length > 0}
		<svelte:self children={child.groups} {handleTileClick} parent_uuid={child.uuid} {depth} />
	{/if}
{/each}

<style>
</style>
