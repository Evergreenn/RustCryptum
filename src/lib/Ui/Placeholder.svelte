<script>
	export let color = '';
	let classCircle = '';
	let classButton = '';
	let classCard = '';
	let classPlacholder = '';

	classCircle = `placeholder-circle w-12 ${color}-border ${color}-placeholder`;
	classButton = `placeholder ${color}-background`;
	classCard = `card ${color}-card`;
	classPlacholder = `placeholder ${color}-placeholder`;
</script>

<section class={classCard}>
	<div class="p-2 space-y-4">
		<div class={classCircle} />
		<div class="grid grid-cols gap-2">
			<div class={classPlacholder} />
			<div class={classPlacholder} />
		</div>
		<div class="grid grid-cols-4 gap-4">
			<div class={classPlacholder} />
			<div class={classPlacholder} />
			<div class={classPlacholder} />
			<div class={classButton} />
		</div>
	</div>
</section>

<style>
	.crimson-placeholder {
		background-color: rgb(45, 39, 51);
	}
	.crimson-card {
		background-color: rgb(22, 19, 25);
	}
	.crimson-border {
		border: 1px solid rgb(214, 86, 73);
	}
	.crimson-background {
		background-color: rgb(214, 86, 73);
	}

	.tealLightning-border {
		border: 1px solid rgb(134, 185, 176); /* color-primary-500 */
	}
	.tealLightning-background {
		background-color: rgb(134, 185, 176); /* color-primary-500 */
	}
	.tealLightning-card {
		background-color: rgb(2, 19, 24); /* color-surface-900 */
	}
	.tealLightning-placeholder {
		background-color: rgb(4, 38, 48); /* color-surface-500 */
	}

	.modern-border {
		border: 1px solid #ec4899; /* color-primary-500 */
	}
	.modern-background {
		background-color: #ec4899; /* color-primary-500 */
	}
	.modern-card {
		background-color: #313276; /* color-surface-900 */
	}
	.modern-placeholder {
		background-color: #6366f1; /* color-surface-500 */
	}

	.wintry-border {
		border: 1px solid #3b82f6; /* color-primary-500 */
	}
	.wintry-background {
		background-color: #3b82f6; /* color-primary-500 */
	}
	.wintry-card {
		background-color: #111827; /* color-surface-900 */
	}
	.wintry-placeholder {
		background-color: #6b7280; /* color-surface-500 */
	}

	.vintage-border {
		border: 1px solid #ea861a; /* color-primary-500 */
	}
	.vintage-background {
		background-color: #ea861a; /* color-primary-500 */
	}
	.vintage-card {
		background-color: #1f1b18; /* color-surface-900 */
	}
	.vintage-placeholder {
		background-color: #3f3731; /* color-surface-500 */
	}
</style>
