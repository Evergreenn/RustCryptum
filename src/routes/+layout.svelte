<script lang="ts">
	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import { Toast } from '@skeletonlabs/skeleton';
	import { Modal } from '@skeletonlabs/skeleton';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });
</script>

<Modal />
<Toast position={'tr'} duration={500} />
<slot />
