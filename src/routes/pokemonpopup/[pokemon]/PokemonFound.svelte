<script lang="ts">
  export let pokemon_id:number;
  import { invoke } from '@tauri-apps/api/tauri';
  import type { PokemonResult } from './PokemonTypes.svelte';

  let pokemon:PokemonResult;

  const get_pokemon = async (id:any) => {
    let pokemon_fetch:PokemonResult= await invoke('get_pokemon',  { pokemonId: id});   
    console.log(pokemon_fetch);
    pokemon = pokemon_fetch;
  }

  get_pokemon(pokemon_id)
</script>


<div>
  {#if pokemon != null}
    <div class="px-8 py-6">
      <h1 class="font-bold text-4xl first-letter:uppercase font-sane">{pokemon.pokemon.name}</h1>
      <div class="border rounded-md mt-5 flex">
        <img 
        src="/sprites/pokemon/{pokemon.pokemon.id}.png" 
        alt={pokemon.pokemon.name} 
        class="object-contain w-80"
        /> 
        <div>
          <h2 class="font-semibold text-xl">
            Information 
          </h2>
          <p class="mt-4">
            Height:  {pokemon.pokemon_information.height}.0
          </p>
          <p>
            Weight:  {pokemon.pokemon_information.weight}.0
          </p>
          <p>
            Base experience:  {pokemon.pokemon_information.base_experience}.0
          </p>
          <audio
            controls
            src="https://play.pokemonshowdown.com/audio/cries/{pokemon.pokemon.name}.mp3">
          </audio>

          <p class="mt-5">Abilities:</p>
          {#each pokemon.pokemon_information.abilities  as ability_info  }
            <p class="border bg-green-500 text-white w-24 text-center rounded p-0.5 m-1" >{ability_info.ability.name}</p>
          {/each}
        </div>
      </div>
    </div>
    {:else}
    <progress class="progress w-[100vw]"></progress>
  {/if}
</div>
