<script lang="ts">
	import { fade } from 'svelte/transition';
	import { Store } from 'tauri-plugin-store-api';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import { invoke } from '@tauri-apps/api/tauri';

	import ButtonsList from '../lib/ButtonsList.svelte';
	import Button from '../lib/Button.svelte';
	import TextInput from '../lib/TextInput.svelte';
	import Textarea from '../lib/Textarea.svelte';

	import { FormValues } from '../enums/form-values.enum';
	import { MenuType } from '../enums/menu-type.enum';
	import type { ScriptButton } from '../types/script-button.interface';

	import '../styles/input.scss';
	import '../styles/shared.scss';

	const scriptStore = writable<ScriptButton[] | null>(null);
	const store = new Store('.config.dat');

	let buttons: ScriptButton[] = [];
	let menu: Record<MenuType, boolean> = {
		[MenuType.AddButton]: false,
		[MenuType.RemoveButton]: false
	};
	let formValues = { [FormValues.Title]: '', [FormValues.Script]: '' };
	let selectedButtons: string[] = [];

	const closeMenu = () => (menu = { [MenuType.AddButton]: false, [MenuType.RemoveButton]: false });

	const toggleMenu = async (type: MenuType) => {
		selectedButtons = [];

		const newMenuState = {
			[type]: !menu[type],
			[type === MenuType.AddButton ? MenuType.RemoveButton : MenuType.AddButton]: false
		};
		menu = { ...menu, ...newMenuState };
	};

	const removeSelectedButtons = async () => {
		await Promise.all(selectedButtons.map(async (id) => await store.delete(id)));
		fetchButtonStoreData();
		closeMenu();
		selectedButtons = [];
	};

	const onSelectButton = (id: string) => {
		if (selectedButtons.includes(id)) {
			selectedButtons = selectedButtons.filter((buttonId) => buttonId !== id);
			return;
		}

		selectedButtons = [...selectedButtons, id];
	};

	const handleSubmit = async () => {
		const { [FormValues.Title]: title, [FormValues.Script]: script } = formValues;
		const uniqueId = crypto.randomUUID();
		await store.set(uniqueId, { text: title, script });
		await store.save();

		formValues = { [FormValues.Title]: '', [FormValues.Script]: '' };
		fetchButtonStoreData();
		closeMenu();
	};

	const handleScriptButtonClick = async ({ detail }: CustomEvent<{ button: ScriptButton }>) => {
		const { id, script } = detail.button;

		if (menu[MenuType.RemoveButton]) onSelectButton(id);
		else await invoke('run_script', { script });
	};

	const fetchButtonStoreData = async () => {
		const entries = (await store.entries()) as [string, ScriptButton][];
		scriptStore.set(entries.map(([id, { script, text }]) => ({ id, text, script })));
	};

	scriptStore.subscribe((values) => (buttons = values || []));

	onMount(fetchButtonStoreData);
</script>

<div class="container">
	<div class="grid">
		<Button text="Add button" on:click={() => toggleMenu(MenuType.AddButton)} />
		<Button
			text="Remove button"
			disabled={!buttons?.length}
			on:click={() => toggleMenu(MenuType.RemoveButton)}
		/>
	</div>

	{#if menu[MenuType.AddButton]}
		<form class="menu-add-button" on:submit|preventDefault={handleSubmit} in:fade>
			<TextInput
				class="input-element"
				placeholder="Add title"
				bind:value={formValues[FormValues.Title]}
			/>
			<Textarea
				class="input-element"
				placeholder="Paste script"
				bind:value={formValues[FormValues.Script]}
			/>
			<Button
				type="submit"
				text="Save"
				disabled={!formValues[FormValues.Title] || !formValues[FormValues.Script]}
			/>
		</form>
	{:else if menu[MenuType.RemoveButton]}
		<div class="menu-remove-button grid" in:fade>
			<p>Select buttons to remove</p>
			{#if !!selectedButtons.length}
				<Button on:click={removeSelectedButtons} text="Remove selected" />
			{/if}
		</div>
	{/if}

	<ButtonsList
		class="buttons-list"
		selectable={!!menu[MenuType.RemoveButton]}
		{buttons}
		{selectedButtons}
		on:click={handleScriptButtonClick}
	/>
</div>

<style lang="scss">
	@import url('htpps://fonts.googleapis.com/css?family=Poppins:100,200,300,400,500,600,700,800,900');

	:global(body) {
		margin: 0;
		padding: 0;
		color: #e0ebeb;
		box-sizing: border-box;
		font-size: 16px;
		font-family: 'Poppins', sans-serif;
		background: #142d4c;
	}

	.container {
		padding: 2rem;

		.menu-add-button {
			display: grid;
			gap: 0.5rem 0;
			margin-top: 2rem;
		}

		.menu-remove-button {
			align-items: baseline;
			margin-top: 2rem;
		}
	}

	.container :global(.buttons-list) {
		margin-top: 1rem;
		border-width: 1px 0 0 0;
		border-style: solid;
		border-color: #385170;
		padding: 2rem 0;
	}
</style>
