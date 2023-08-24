<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressRadial, modalStore, toastStore } from '@skeletonlabs/skeleton';
	import PasswordOptions from './PasswordOptions.svelte';
	import { useStateStore } from '$lib/stores/stateStore';

	let keyName: string = '';
	let userName: string = '';
	let password: string = '';
	let url: string = '';
	let length: number;
	let useSymbols: boolean;
	let useSpaces: boolean;
	let useNumbers: boolean;
	let useUppercase: boolean;
	let useLowercase: boolean;
	let useExcludeSimilarCharacters: boolean;

	const stateStore = useStateStore();

	let isLoading: boolean = false;

	const onClick = () => {
		isLoading = true;

		invoke('create_new_key', {
			name: keyName,
			password,
			currentGroup: $stateStore.breadcrumb,
			username: userName,
			url
		})
			.then((_res) => {
				// if (res === true) {
				isLoading = false;
				modalStore.close();
				let toast = {
					message: 'New key created successfully',
					background: 'variant-filled-success'
				};
				toastStore.trigger(toast);
				// }
			})
			.catch((err) => {
				isLoading = false;
				console.log(err);
				let toast = {
					message: 'Error creating new key',
					background: 'variant-filled-error'
				};
				toastStore.trigger(toast);
			});
	};

	const onHandlePasswordGeneration = async () => {
		const res: string = await invoke('generate_password', {
			options: {
				length,
				use_symbols: useSymbols,
				use_spaces: useSpaces,
				use_numbers: useNumbers,
				use_uppercase: useUppercase,
				use_lowercase: useLowercase,
				exclude_similar_characters: useExcludeSimilarCharacters
			}
		});
		password = res;
	};
</script>

<div class="card p-8 w-1/2">
	<form class="relative pb-16">
		<label class="label mb-6">
			<span>Key name</span>
			<input bind:value={keyName} class="input" type="text" placeholder="Random Key Name" />
		</label>
		<label class="label mb-6">
			<span>Username</span>
			<input bind:value={userName} class="input" type="text" placeholder="Username" />
		</label>
		<label class="label mb-6">
			<span>URL</span>
			<input bind:value={url} class="input" type="text" placeholder="URL" />
		</label>
		<div class="flex flex-row gap-6 w-full">
			<label class="label w-5/6">
				<span>Password</span>
				<input bind:value={password} class="input" type="text" placeholder="Password" />
			</label>
			<div class="flex items-end justify-end">
				<button
					on:click={onHandlePasswordGeneration}
					type="button"
					class="btn variant-ghost-primary">Generate Password</button
				>
			</div>
			<label class="label">
				<button
					on:click={onClick}
					type="submit"
					class="btn variant-ghost-primary absolute bottom-0 right-0"
				>
					{#if isLoading}
						<ProgressRadial width={'w-4'} stroke={40} value={undefined} />
					{:else}
						Generate
					{/if}
				</button>
			</label>
		</div>
		<PasswordOptions
			bind:length
			bind:useSymbols
			bind:useSpaces
			bind:useNumbers
			bind:useUppercase
			bind:useLowercase
			bind:useExcludeSimilarCharacters
		/>
	</form>
</div>
