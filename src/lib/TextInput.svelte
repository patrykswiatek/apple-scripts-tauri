<script lang="ts">
	export let placeholder: string;
	export let value: string;
</script>

<input class={$$restProps.class || ''} {placeholder} bind:value type="text" />
