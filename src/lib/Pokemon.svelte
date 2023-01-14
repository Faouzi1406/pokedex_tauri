<script lang="ts">
  import type PokemonInterface  from "./PokemonInterface.svelte";

  export let pokemon_i:PokemonInterface;
</script>

<div class="rounded-md shadow-md p-2 flex-col text-center w-80 border border-gray-400">
  <img src="sprites/pokemon/{pokemon_i.id}.png" alt={pokemon_i.name} class="w-full object-cover"/> 
  <p class="font-semibold">{pokemon_i.name}</p>
</div>
