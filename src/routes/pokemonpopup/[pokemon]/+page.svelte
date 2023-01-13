<script lang="ts">
  import PokemonNotFound from './PokemonNotFound.svelte';
  import PokemonFound from './PokemonFound.svelte';

  type PokemonRequest = { 
    pokemon_id:number,
    error:string
  }

  export let data:PokemonRequest;
</script>

<div class="w-[100vw]">
  {#if data.error != "none"}
    <PokemonNotFound />
    {:else}
    <PokemonFound pokemon_id={data.pokemon_id}/>
  {/if}
</div>
