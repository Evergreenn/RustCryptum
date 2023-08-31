<script lang="ts">
	import { goto } from '$app/navigation';
	import BackButton from '$lib/Ui/Button/BackButton.svelte';
	import ThemeSwitcher from '$lib/Ui/Button/ThemeSwitcher.svelte';
	import Title from '$lib/Ui/Title/Title.svelte';

	const onClick = () => {
		goto('/home');
	};
</script>

<div class="flex flex-row justify-between">
	<BackButton />
</div>

<Title divider={true} title="Change theme" weight={'h1'} />
<div class={'mt-4'}>
	<ThemeSwitcher />
</div>
