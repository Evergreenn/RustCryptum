<script lang="ts">
	import PasswordOptions from '$lib/Forms/PasswordOptions.svelte';
	import Folder from '$lib/icons/Folder.svelte';
	import { useStateStore } from '$lib/stores/stateStore';

	export let parent_uuid: number = 0;
	export let depth: number = 2;
	export let children: any = [];
	export let handleTileClick: (e: any, uuid: number) => void;
	let statestore = useStateStore();

	const setBreadcrumb = (title: string) => {
		$statestore.breadcrumb = title;
	};

	$: {
		if (parent_uuid) {
			depth = depth + 1;
		} else {
			depth = 2;
		}
	}
</script>

{#each children as child (child.uuid)}
	{#if parent_uuid}
		<li class="ml-{depth * 2}">
			<a
				href="/main/{child.uuid}"
				on:click={() => {
					setBreadcrumb(child.name);
				}}
			>
				<span class="badge bg-primary-500">
					<Folder />
				</span>
				<span class="flex-auto">{child.name}</span>
			</a>
		</li>
	{:else}
		<li class="ml-{depth / 2}">
			<a
				href="/main/{child.uuid}"
				on:click={() => {
					setBreadcrumb(child.name);
				}}
			>
				<span class="badge bg-primary-500">
					<Folder />
				</span>
				<span class="flex-auto">{child.name}</span>
			</a>
		</li>
	{/if}
	{#if child.groups.length > 0}
		<svelte:self children={child.groups} {handleTileClick} parent_uuid={child.uuid} {depth} />
	{/if}
{/each}

<style>
</style>
