<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { modalStore } from '@skeletonlabs/skeleton';
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

	const onClick = async () => {
		const res = await invoke('create_new_key', {
			name: keyName,
			password,
			currentGroup: $stateStore.breadcrumb,
			username: userName,
			url
		});
		console.log(res);

		modalStore.close();
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

<div class="card p-8 w-full">
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
					class="btn variant-ghost-primary absolute bottom-0 l weft-0 right-0">Generate</button
				>
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
