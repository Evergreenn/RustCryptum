<script lang="ts">
	import { RangeSlider, SlideToggle } from '@skeletonlabs/skeleton';

	const max_length = 128;

	interface PasswordOptions {
		length: number;
		use_symbols: boolean;
		use_spaces: boolean;
		use_numbers: boolean;
		use_uppercase: boolean;
		use_lowercase: boolean;
		exclude_similar_characters: boolean;
	}

	let passwordDefaults: PasswordOptions = {
		length: 32,
		use_symbols: true,
		use_spaces: false,
		use_numbers: true,
		use_uppercase: true,
		use_lowercase: true,
		exclude_similar_characters: true
	};

	export let length = passwordDefaults.length;
	export let useSymbols = passwordDefaults.use_symbols;
	export let useSpaces = passwordDefaults.use_spaces;
	export let useNumbers = passwordDefaults.use_numbers;
	export let useUppercase = passwordDefaults.use_uppercase;
	export let useLowercase = passwordDefaults.use_lowercase;
	export let useExcludeSimilarCharacters = passwordDefaults.exclude_similar_characters;
</script>

<div class="w-full h-60 flex flex-col pt-4">
	<RangeSlider
		class="mt-4 mb-8"
		name="range-slider"
		bind:value={length}
		min={6}
		max={max_length}
		step={1}
		ticked
		accent={'accent-primary-500 dark:accent-primary-500'}
	>
		<div class="flex justify-between items-center">
			<div class="font-bold">Password length</div>
			<div class="text-xs">{length} / {max_length}</div>
		</div>
	</RangeSlider>
	<!-- <div class="flex flex-row pt-4"> -->
	<div class="grid grid-cols-1 gap-6 sm:grid-cols-1 md:grid-cols-1 lg:grid-cols-1">
		<!-- <div class="flex flex-col w-1/2 gap-4"> -->
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useSymbols}
			>use Symbols (e.g. !@#$%^&*)</SlideToggle
		>
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useSpaces}
			>use Spaces</SlideToggle
		>
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useNumbers}
			>use Numbers</SlideToggle
		>
		<!-- </div> -->
		<!-- <div class="flex flex-col w-1/2 gap-4"> -->
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useUppercase}
			>use Uppercase Characters</SlideToggle
		>
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useLowercase}
			>use Lowercase Characters</SlideToggle
		>
		<SlideToggle name="slide" active={'bg-primary-500'} bind:checked={useExcludeSimilarCharacters}
			>exclude Similar Characters</SlideToggle
		>
		<!-- </div> -->
	</div>
</div>
