<script lang="ts">
	export let placeholder: string;
	export let value: string;
	export let rows = 6;
</script>

<textarea class={$$restProps.class || ''} {placeholder} bind:value {rows} />
