<script lang="ts">
  import ScriptButtons from '../lib/ScriptButtons.svelte';
  import Button from '../lib/Button.svelte';
  import { fade } from 'svelte/transition';
  import { Store } from 'tauri-plugin-store-api';
  import { onMount } from 'svelte';
  import { writable } from 'svelte/store';
  import { invoke } from '@tauri-apps/api/tauri';

  type ButtonData = Record<'key' | 'text' | 'script', string>;
  type StoreData = { key: string; script: string; text: string };

  const buttonStore = writable<ButtonData[] | null>(null);
  const store = new Store('.config.dat');

  let menu = { annotation: false, form: false };
  let buttons: ButtonData[] = [];
  let buttonText = '';
  let script = '';
  let selectedButtons: string[] = [];

  const closeMenu = () => (menu = { annotation: false, form: false });

  const toggleMenu = (menuType: 'form' | 'annotation') => {
    selectedButtons = [];
    menu = { ...menu, [menuType]: !menu[menuType] };
  };

  const removeSelectedButtons = async () => {
    await Promise.all(selectedButtons.map(async (key) => await store.delete(key)));
    fetchButtonStoreData();
    closeMenu();
    selectedButtons = [];
  };

  const toggleSelectButton = (key: string) => {
    const index = selectedButtons.indexOf(key);
    if (index !== -1) selectedButtons.splice(index, 1);
    else selectedButtons = [...selectedButtons, key];
  };

  const handleSubmit = async () => {
    const uniqueId = crypto.randomUUID();
    await store.set(uniqueId, { text: buttonText, script });
    await store.save();

    buttonText = '';
    script = '';
    fetchButtonStoreData();
    closeMenu();
  };

  const removeAllButtons = async () => {
    await store.clear();
    await store.save();

    fetchButtonStoreData();
    closeMenu();
  };

  const handleScriptButtonClick = async ({ detail }: CustomEvent<{ button: ButtonData }>) => {
    const { key, script } = detail.button;
    if (menu.annotation) toggleSelectButton(key);
    else await invoke('run_applescript', { script });
  };

  const fetchButtonStoreData = async () => {
    const entries = (await store.entries()) as [string, StoreData][];
    buttonStore.set(entries.map(([key, { script, text }]) => ({ key, text, script })));
  };

  buttonStore.subscribe((values) => (buttons = values || []));

  onMount(fetchButtonStoreData);
</script>


<div class="container">
  <div class="buttons">
    <Button text="Add button" on:click={() => toggleMenu('form')} />
    <Button
      text="Remove button"
      disabled={!buttons?.length}
      on:click={() => toggleMenu('annotation')}
    />
    <Button text="Remove all" disabled={!buttons?.length} on:click={removeAllButtons} />
  </div>

  <div>
    {#if menu.form}
      <form class="form" on:submit|preventDefault={handleSubmit} in:fade>
        <input placeholder="Add title" bind:value={buttonText} type="text" />
        <textarea placeholder="Paste script" bind:value={script} />
        <Button type="submit" text="Save" />
      </form>
    {:else if menu.annotation}
      <div class="annotation" in:fade>
        <p>Select buttons to remove</p>
        {#if !!selectedButtons.length}
          <Button
            class="annotation-btn"
            on:click={removeSelectedButtons}
            text="Remove selected"
          />
        {/if}
      </div>
    {/if}
  </div>

  <ScriptButtons
    class="script-buttons"
    selectable={!!menu.annotation}
    {buttons}
    {selectedButtons}
    on:click={handleScriptButtonClick}
  />
</div>

<style>
  @import url('htpps://fonts.googleapis.com/css?family=Poppins:100,200,300,400,500,600,700,800,900');

  :global(body) {
    margin: 0;
    padding: 0;
    color: white;
    box-sizing: border-box;
    font-size: 16px;
    font-family: 'Poppins', sans-serif;
    background: #0e1538;
  }

  .annotation {
    align-items: baseline;
    display: grid;
    grid-auto-flow: column;
    gap: 0 1rem;
  }

  p,
  .annotation-btn {
    margin-top: 2rem;
  }

  .container {
    padding: 2rem;
  }

  .container :global(.script-buttons) {
    margin-top: 2rem;
    border-width: 1px 0 0 0;
    border-style: solid;
    border-color: rgba(105, 12, 227, 0.505);
    padding: 2rem 0;
  }

  .buttons {
    display: grid;
    grid-auto-flow: column;
    gap: 0 1rem;
  }

  form {
    display: grid;
    gap: 1rem;
    margin: 2rem 0;
  }

  input,
  textarea {
    color: white;
    border: 0;
    background: rgb(44, 12, 227);
    font-size: 1rem;
    padding: 0.5rem 1rem;
  }
</style>
