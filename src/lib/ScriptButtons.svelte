<script lang="ts">
	import Button from './Button.svelte';
	import { createEventDispatcher } from 'svelte';

	export let buttons: Record<'key' | 'text' | 'script', string>[];
	export let selectable: boolean;
	export let selectedButtons: string[];

	const dispatch = createEventDispatcher<{ click: { button: (typeof buttons)[number] } }>();

</script>

<div class={`${$$restProps.class || ''} script-buttons`}>
	{#each buttons as button (`${button.key}-${selectable}-${selectedButtons}`)}
		<Button
			{selectable}
			text={button.text}
			selected={selectedButtons.includes(button.key)}
			on:click={() => dispatch('click', { button })}
		/>
	{/each}
</div>

<style>
	.script-buttons {
		display: grid;
		grid-template-columns: auto auto;
		gap: 1rem;
	}
</style>
