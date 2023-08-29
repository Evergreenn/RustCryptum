<script lang="ts">
	import { getModalStore } from '@skeletonlabs/skeleton';
	import AddNewKey from '$lib/Forms/AddNewKey.svelte';

	const modalStore = getModalStore();
	const displayModal = () => {
		const modalComponent = {
			ref: AddNewKey
			// props: {
			//   title: 'Add New Key',
			//   body: 'Provide your first name in the field below.',
			//   value: 'Skeleton',
			//   valueAttr: { type: 'text', minlength: 3, maxlength: 10, required: true },
			// response: (r: string) => console.log('response:', r)
			// }
		};

		const modal = {
			type: 'component',
			component: modalComponent,
			// backdropClasses: '!bg-green-500',
			modalClasses: '!bg-red-500',
			// Data
			// title: 'Enter Name',
			// body: 'Provide your first name in the field below.',
			// Populates the input value and attributes
			// value: 'Skeleton',
			// valueAttr: { type: 'text', minlength: 3, maxlength: 10, required: true },
			// Returns the updated response value
			response: (r: string) => console.log('response:', r)
		};
		modalStore.trigger(modal);
		// console.log('display modal');
	};
</script>

<div class="absolute bottom-0">
	<button on:click={displayModal} type="button" class="btn variant-filled">
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
					d="M12 9v6m3-3H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z"
				/>
			</svg>
		</span>
		<span>Add a key</span>
	</button>
</div>
