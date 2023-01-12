<script lang="ts">
  import { WebviewWindow } from '@tauri-apps/api/window'
  import PokemonNotFound from './PokemonNotFound.svelte';
  import PokemonFound from './PokemonFound.svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  //Interface voor de pokemon PokemonRequest 
  //Heeft een pokemon_id
  //Let op pokemon id kan null zijn als er geen pokemon id is mee gegeven
  interface PokemonRequest {
    pokemon_id:number,
    error:string
  }

  const get_pokemon = async (id:any) => {
    let pokemon = await invoke('get_pokemon',  { pokemonId: id});   
  }

  export let data:PokemonRequest;

  get_pokemon(data.pokemon_id);
</script>

<div class="w-[100vw]">
  {#if data.error != "none"}
    <PokemonNotFound />
    {:else}
    <PokemonFound pokemon_id={data.pokemon_id}/>
  {/if}
</div>
