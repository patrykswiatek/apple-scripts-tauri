<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	export let text: string;
	export let selectable = false;
	export let selected = false;
	export let disabled = false;
	export let type: HTMLButtonElement['type'] = 'button';
	export let variant: 'primary' | 'secondary' = 'primary';

	$: buttonClass = [
		'button',
		selectable && 'button--selectable',
		selected && 'button--selected',
		$$restProps.class || ''
	]
		.filter(Boolean)
		.join(' ');

	const dispatch = createEventDispatcher();
</script>

<div class={`button-wrapper button-wrapper--${variant}`}>
	<button class={buttonClass} {type} {disabled} on:click={() => dispatch('click')}>
		{text}
	</button>
</div>

<style lang="scss">
	.button-wrapper {
		.button {
			background-color: #9fd3c7;
			border: none;
			padding: 0.5rem 1rem;
			text-align: center;
			color: #142d4c;
			font-size: 1rem;
			text-decoration: none;
			display: inline-block;
			cursor: pointer;
			transition-duration: 0.4s;
			width: 100%;

			&:disabled {
				opacity: 0.6;
				cursor: not-allowed;
			}

			&--selectable {
				background-color: transparent;
				border: 1px solid #9fd3c7;
				color: #9fd3c7;
			}
		}

		&--secondary {
			.button {
				background: #49beb7;
				color: #142d4c;

				&--selectable {
					background-color: transparent;
					border: 1px solid #49beb7;
					color: #49beb7;
				}

				&--selected {
					background-color: #f95959;
					border: none;
					color: #ececec;
				}
			}
		}
	}
</style>
