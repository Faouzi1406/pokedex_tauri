<script lang="ts">
  import { WebviewWindow } from '@tauri-apps/api/window'
  import PokemonNotFound from './PokemonNotFound.svelte';
  export let data:any;
</script>

<div class="w-[100vw]">
  {#if data.error != "none"}
    <PokemonNotFound />
    {:else}
  {/if}
</div>
