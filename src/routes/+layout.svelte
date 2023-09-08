<script lang="ts">
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { Modal } from '@skeletonlabs/skeleton';
	import { Toast } from '@skeletonlabs/skeleton';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';

	initializeStores();
	// getToastStore();
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	invoke('get_config').then((config: any) => {
		const theme = config.color_scheme.toLowerCase();
		const body = document.body;
		body.dataset.theme = theme;
	});
</script>

<Modal />
<Toast position={'tr'} class={'top-32'} duration={200} />
<slot />
