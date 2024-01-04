<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let text: string;
  export let selectable = false;
  export let selected = false;
  export let disabled = false;
  export let type: HTMLButtonElement['type'] = 'button';

  const buttonClass = [
    selectable && 'selectable',
    selected && 'selected',
    'btn',
    $$restProps.class || ''
  ].filter(Boolean).join(' ');

  const dispatch = createEventDispatcher();
</script>

<button class={buttonClass} {type} {disabled} on:click={() => dispatch('click')}>
  {text}
</button>

<style>
  .btn {
    background-color: rgb(44, 12, 227);
    border: none;
    color: white;
    padding: 0.5rem 1rem;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 1rem;
    cursor: pointer;
    transition-duration: 0.4s;

    &:disabled {
      opacity: 0.6;
      cursor: not-allowed;
    }
  }

  .selectable {
    background-color: transparent;
    border: 1px solid rgb(44, 12, 227);
    color: rgb(44, 12, 227);
  }

  .selected {
    background-color: rgb(255, 82, 82);
    border: none;
    color: white;
  }
</style>
