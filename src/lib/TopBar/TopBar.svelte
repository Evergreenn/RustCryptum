<script>
	import { LightSwitch, modalStore } from '@skeletonlabs/skeleton';
	import { Avatar } from '@skeletonlabs/skeleton';
	import Breadcrumb from '$lib/Ui/breadcrumb.svelte';
	import AddNewKey from '$lib/Forms/AddNewKey.svelte';

	const modalComponent = {
		// Pass a reference to your custom component
		ref: AddNewKey,
		// Add the component properties as key/value pairs
		props: { background: 'bg-red-500' },
		// Provide a template literal for the default component slot
		slot: '<p>Skeleton</p>'
		// width: 'w-80'
	};

	const onEntrieCreationHandler = () => {
		const modal = {
			type: 'component',
			// Pass the component directly:
			component: modalComponent
		};
		modalStore.trigger(modal);
	};
</script>

<div class="flex flex-col z-10">
	<div class="flex flex-row">
		<div class="w-1/5">
			<!-- breadcrumb -->
			<div class="p-2">
				<Breadcrumb />
			</div>
			<!-- creation button -->
			<div class="p-2 flex flex-row sm:max-lg:flex-col gap-4">
				<button type="button" class="btn variant-filled-primary" on:click={onEntrieCreationHandler}>
					<span>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							class="w-6 h-6"
						>
							<path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
						</svg>
					</span>
					<span>Add</span>
				</button>

				<button type="button" class="btn variant-filled-primary">
					<span>
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="1.5"
							stroke="currentColor"
							class="w-6 h-6"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M12 10.5v6m3-3H9m4.06-7.19l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
							/>
						</svg>
					</span>
				</button>
				<!-- <button class="bg-primary-500 hover:bg-primary-600 text-white font-bold py-2 px-4 rounded"> -->
				<!-- 	Main -->
				<!-- </button> -->
			</div>
		</div>

		<!-- search bar -->
		<div class="flex w-3/5 justify-center items-center pt-9">
			<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
				<div class="input-group-shim">
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="1.5"
						stroke="currentColor"
						class="w-6 h-6"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
						/>
					</svg>
				</div>
				<input class="variant-glass-surface" type="search" placeholder="Search..." />
				<!-- <button class="variant-filled-secondary">Submit</button> -->
			</div>
			<!-- <input class="input p-3" title="Input (text)" type="text" placeholder="Search" /> -->
		</div>
		<!-- settings - usermenu -->
		<div class="flex flex-row w-1/5 pt-9 items-center place-content-evenly">
			<div class="">
				<LightSwitch />
			</div>
			<div class="">
				<Avatar
					class=""
					src="https://images.unsplash.com/photo-1617296538902-887900d9b592?ixid=M3w0Njc5ODF8MHwxfGFsbHx8fHx8fHx8fDE2ODc5NzExMDB8&ixlib=rb-4.0.3&w=128&h=128&auto=format&fit=crop"
					width="w-14"
					rounded="rounded-full"
				/>
			</div>
		</div>
	</div>
	<hr class="!border-t-4 ml-2 mr-8 mb-4 mt-3" />
</div>
