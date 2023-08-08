<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial, modalStore } from '@skeletonlabs/skeleton';
	import { useStateStore } from '$lib/stores/stateStore';

	let folderName: string = '';
	let isLoading: boolean = false;
	const stateStore = useStateStore();

	const onClick = () => {
		isLoading = true;

		invoke('create_new_folder', {
			name: folderName,
			currentGroup: $stateStore.breadcrumb
		}).then((res) => {
			isLoading = false;
			modalStore.close();
		});
	};
</script>

<div class="card p-8 w-full">
	<form class="relative pb-16">
		<label class="label mb-6">
			<span>Folder name</span>
			<input bind:value={folderName} class="input" type="text" placeholder="Random Folder Name" />
		</label>
		<label class="label">
			<button
				on:click={onClick}
				type="submit"
				class="btn variant-ghost-primary absolute bottom-0 l weft-0 right-0"
			>
				{#if isLoading}
					<ProgressRadial width={'w-4'} stroke={40} value={undefined} />
				{:else}
					Create
				{/if}
			</button>
		</label>
	</form>
</div>
