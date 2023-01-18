<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Pokemon from "./Pokemon.svelte";
  import type  AllPokemon from "./PokemonInterface.svelte";
  import { WebviewWindow } from '@tauri-apps/api/window'
  

  async function create_uuid(){
    let id = await invoke('unique_id');
    console.log(id);
    return id
  }

  let search_query:string = "";
  let search_pokemon:AllPokemon;


  let all_pokemon:AllPokemon;

  const getAllPokemon = async () => {
    let pokemon:AllPokemon = await invoke('get_all_pokemon');
    all_pokemon = pokemon;
    search_pokemon = all_pokemon;
  }

  if (search_query == "") {
     getAllPokemon()
  }

  const searchPokemon = () => {
    let search = all_pokemon.all_pokemon.filter(pokemon => pokemon.name.toLowerCase().includes(search_query.toLowerCase()));

    let all_pokemon_search:AllPokemon = { all_pokemon: [
    {
      id:0,
      name:"placeholder",
      sound:"placeholder",
      image:"placeholder"
    }
    ] };

    search.forEach(pokemon => {
      all_pokemon_search.all_pokemon.push(pokemon);
    });

    search_pokemon = all_pokemon_search;
  }   


  async function create_window(location:string){
    const uuid = await create_uuid();

    const webview = new WebviewWindow(`${uuid}`, {
        url: `./pokemonpopup/${location}`,
    })

    webview.once('tauri://error', function (e) {
      console.log(e)
    });
  }

  getAllPokemon()
</script>

<header class="w-[100vw] p-2">

<div class="navbar bg-base-100">
  <div class="flex-1">
    <a class="btn btn-ghost normal-case text-xl" href="/">Pokedex</a>
  </div>
  <div class="flex-none gap-2">
    <div class="form-control">
      <input 
      type="text" 
      placeholder="Search" 
      class="input input-bordered" 
      bind:value={ search_query }  
      on:input={  searchPokemon } />
    </div>
    <div class="dropdown dropdown-end">
      <label tabindex="0" class="btn btn-ghost btn-circle avatar">
        <div class="w-10 rounded-full">
          <img src="/icon.png" alt="pokedex icon"/>
        </div>
      </label>
      <ul tabindex="0" class="mt-3 p-2 shadow menu menu-compact dropdown-content bg-base-100 rounded-box w-52">
        <li>
          <a class="justify-between" href="/mycollection">
            Collection
          </a>
        </li>
      </ul>
    </div>
  </div>
</div>

</header>

<div class="mb-2">
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-screen  place-items-center" >
    {#if all_pokemon !=null }
      {#each search_pokemon.all_pokemon as pokemon}
        {#if pokemon.id < 900 && pokemon.id >0}
        <div on:click={() => create_window(pokemon.id)}>
          <Pokemon pokemon_i={pokemon}   /> 
        </div>
        {/if}
      {/each}
    {/if} 
  </div>
</div>


