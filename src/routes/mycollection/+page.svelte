<script lang="ts"> 
  import Pokemon from "$lib/Pokemon.svelte";
  import { Store } from "tauri-plugin-store-api";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { AllPokemon, PokemonInterface } from "$lib/PokemonInterface.svelte";
  import { WebviewWindow } from "@tauri-apps/api/window";

  let totalpokemon:number = 0;

  const storePokemon = async  (pokemon_name:string) => {
    let store = new Store('pokemon-stored.pokedex');
    await store.set(pokemon_name, { name: pokemon_name });
  }   

  async function create_uuid(){
    let id = await invoke('unique_id');
    console.log(id);
    return id
  }

  const getAllStored = async ():Promise<string[]> => {
    let store = new Store('pokemon-stored.pokedex');
    let all_stored = await store.keys();
    return all_stored
  }

  let search_query:string = "";
  let all_pokemon:AllPokemon;
  let search_pokemon:AllPokemon;

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

  const savePokemon = async (pokemon_div:MouseEvent) => {
    let pokemonName = pokemon_div.target.id.replace("\n", "");
    storePokemon(pokemonName);
    getSavedPokemon();
  }

  let savedPokemon:AllPokemon;
  let searchSavedPokemonValue:AllPokemon;
  let searchSavedQuery:string = '';

  const getSavedPokemon = async () => {
    let pokemonAll = await getAllStored();
    totalpokemon = pokemonAll.length;
    let pokemonSaved:AllPokemon = await invoke('get_stored_pokemon', { pokemons:pokemonAll });
    savedPokemon = pokemonSaved;
    searchSavedPokemonValue = pokemonSaved;
  }

  const searchSavedPokemon = () => {
    let search = savedPokemon.all_pokemon.filter(pokemon => pokemon.name.toLowerCase().includes(searchSavedQuery.toLowerCase()));

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

    searchSavedPokemonValue = all_pokemon_search;
  }

  getSavedPokemon()


  async function create_window(location:number){
    const uuid = await create_uuid();

    const webview = new WebviewWindow(`${uuid}`, {
        url: `./pokemonpopup/${location}`,
    })

    webview.once('tauri://error', function (e) {
      console.log(e)
    });
  }

  
</script>

<header class="w-[100vw] p-2">

<div class="navbar bg-base-100">
  <div class="flex-1">
    <a class="btn btn-ghost normal-case text-xl" href="/">Pokedex</a>
  </div>
  <div class="flex-none gap-2">
    <div class="form-control">
      <input type="text" 
      placeholder="Search" 
      class="input input-bordered" 
      bind:value={ searchSavedQuery  }  
      on:input={ searchSavedPokemon }
      />
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

<div class=" px-8"> 

<div class="flex items-center">
  <div class="grow">
    <label for="my-modal" class=" bg-primary-focus p-3 rounded text-xl font-bold">Add pokemon</label>
  </div>
  <p class="font-semibold text-xl">Collection count: { totalpokemon } </p>
</div>

<input type="checkbox" id="my-modal" class="modal-toggle" />
<div class="modal">
  <div class="modal-box">
    <h3 class="font-bold text-lg">Add pokemon</h3>
    <p class="py-4">een pokemon</p>
    <input 
    class="rounded-md p-2 h-10 w-full border mb-2" 
    placeholder="Zoek" 
    bind:value={ search_query }
    on:input={ searchPokemon }
    />


    <!-- search -->
    <div class="h-28 overflow-auto ">
      {#if all_pokemon !=null }
      {#each search_pokemon.all_pokemon as pokemon}
        {#if pokemon.id < 900 && pokemon.id != 0}
        <div 
        class="m-2 flex items-center gap-2 font-semibold  rounded p-1 hover:cursor-pointer hover:border border-info-content"  
        aria-label={pokemon.name}
        on:click={(e)=> savePokemon(e)}
        id={pokemon.name}
        >
          <img 
          src="sprites/pokemon/{pokemon.id}.png" 
          alt={pokemon.name} 
          class="rounded-full object-contain border w-10 aspect-square"
          id={pokemon.name}/> 
          <p id={pokemon.name}>{pokemon.name}</p>
        </div>
        {/if}
      {/each}
    {/if}
    </div>
    <div class="modal-action">
      <label  for="my-modal" class="btn btn-error">Close</label>
    </div>
  </div>
</div>
   <div class="mb-2 mt-5">
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 w-screen  place-items-center" >
        {#if savedPokemon !=null }
          {#each searchSavedPokemonValue.all_pokemon as pokemon}
           {#if pokemon.id < 900 && pokemon.id > 0}
            <div on:click={() => create_window(pokemon.id)}>
             <Pokemon pokemon_i={pokemon} /> 
            </div>
           {/if}
          {/each}
        {/if} 
      </div>
    </div>

    <!-- added pokemon succes -->

    <div>

    </div>
</div>
