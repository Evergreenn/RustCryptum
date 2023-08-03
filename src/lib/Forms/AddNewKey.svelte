<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { modalStore } from '@skeletonlabs/skeleton';
	let keyName = '';

	const onClick = async () => {
		const res = await invoke('create_new_key', { name: keyName });
		console.log(res);

		modalStore.close();
	};
</script>

<div class="card p-8">
	<form class="relative pb-16">
		<label class="label mb-6">
			<span>Key name</span>
			<input bind:value={keyName} class="input" type="text" placeholder="Random Key Name" />
		</label>
		<label class="label">
			<button
				on:click={onClick}
				type="submit"
				class="btn variant-ghost-primary absolute bottom-0 left-0 right-0">Generate</button
			>
		</label>
	</form>
</div>
