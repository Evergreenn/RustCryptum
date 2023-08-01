<script lang="ts">
</script>

<div class="card variant-glass-surface">
	<header class="card-header">
		<div class="flex flex-col">
			<div class="">
				<div class="">
					<div class="flex flex-row items-center justify-between">
						<div class="flex flex-row items-center justify-start">
							<!-- <slot name="subtitle"> -->
							<slot name="title">
								<p class="card-header-title">Component</p>
							</slot>
							<!-- 	<p class="card-header-subtitle">Subtitle</p> -->
							<!-- </slot> -->
						</div>
						<div class="flex flex-row items-center justify-end gap-2">
							<slot name="actions">
								<button type="button" class="btn-icon btn-icon-sm variant-ghost-primary">
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
											d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
										/>
									</svg>
								</button>
								<button class="btn-icon btn-icon-sm variant-ghost-primary">
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
											d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z"
										/>
									</svg>
								</button>
							</slot>
						</div>
					</div>
				</div>
			</div>
		</div>
	</header>
	<hr class="!border-t-4 m-4" />
	<section class="p-4">
		<slot name="body">
			<p class="text-sm text-gray-500">Body</p>
		</slot>
	</section>
	<footer class="card-footer mt-4">
		<slot name="footer">
			<p class="text-sm text-gray-500">Footer</p>
		</slot>
	</footer>
</div>
