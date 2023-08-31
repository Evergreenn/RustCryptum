<script lang="ts">
	import { ProgressRadial } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { goto } from '$app/navigation';

	const onHandleLogout = async () => {
		await invoke('lock');
		goto('/');
	};

	onHandleLogout();
</script>

<div class="flex items-center justify-center h-screen p-4">
	<div class="flex flex-col items-center justify-center">
		<div class="mb-4">
			<ProgressRadial size="large" value={undefined} />
		</div>
		<span class="text-2xl">Saving and locking down the database</span>
	</div>
</div>
