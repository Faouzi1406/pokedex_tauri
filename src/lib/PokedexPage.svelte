<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Pokemon from "./Pokemon.svelte";
  import type  AllPokemon from "./PokemonInterface.svelte";
  import { WebviewWindow } from '@tauri-apps/api/window'
  
  let all_pokemon:AllPokemon;

  async function get_all_pokemon() {
    let pokemon:AllPokemon = await invoke('get_all_pokemon');
    all_pokemon = pokemon;
  }

  async function create_window(location:string){
    const webview = new WebviewWindow('', {
        url: `./pokemonpopup/${location}`,
    })
  }

  async function create_uuid(){
    let id = await invoke('unique_id');
    console.log(id);
    return id
  }

  get_all_pokemon()
</script>

<p on:click={create_uuid}>Test uuid</p>

<div class="mb-2">
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-screen  place-items-center" >
    {#if all_pokemon !=null }
      {#each all_pokemon.all_pokemon as pokemon}
        {#if pokemon.id < 900}
        <div on:click={() => create_window("test")}>
          <Pokemon pokemon_i={pokemon}   /> 
        </div>
        {/if}
      {/each}
    {/if} 
  </div>
</div>


