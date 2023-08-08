<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import Card from '$lib/Ui/Card.svelte';
	import { goto } from '$app/navigation';
	import CreateDatabase from '$lib/Forms/CreateDatabase.svelte';
	import { useStateStore } from '$lib/stores/stateStore';

	const state = useStateStore();
	const toggleInit = () => {
		$state.initialized = true;
	};

	$: {
		if ($state.initialized === true) {
			goto('/home');
		}
	}
</script>

<div class="flex items-center justify-center h-screen p-4">
	<Card>
		<span slot="title">Create Database</span>
		<span slot="actions" />
		<span slot="body">
			<CreateDatabase {toggleInit} />
		</span>
		<span slot="footer">
			<div class="flex flex-row items-center justify-center">
				<p><a href="/" class="anchor">Go back</a></p>
			</div>
		</span></Card
	>
</div>
