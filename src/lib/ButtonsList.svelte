<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	import Button from './Button.svelte';

	import type { ScriptButton } from '../types/script-button.interface';

	import '../styles/shared.scss';

	export let buttons: ScriptButton[];
	export let selectable: boolean;
	export let selectedButtons: string[];

	const dispatch = createEventDispatcher<{ click: { button: ScriptButton } }>();
</script>

<ul class={`${$$restProps.class || ''} buttons-list grid`}>
	{#each buttons as button (button.id)}
		<li>
			<Button
				class="btn"
				{selectable}
				text={button.text}
				variant="secondary"
				selected={selectedButtons.includes(button.id)}
				on:click={() => dispatch('click', { button })}
			/>
		</li>
	{/each}
</ul>

<style lang="scss">
	.buttons-list {
		gap: 1rem;
		grid-auto-flow: row;
		list-style: none;
	}
</style>
