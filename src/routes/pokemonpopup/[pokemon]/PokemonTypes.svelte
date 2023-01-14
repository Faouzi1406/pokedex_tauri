<script lang="ts"  context="module">
  import type {PokemonInterface} from "$lib/PokemonInterface.svelte";

  type pokemonSlug = {
    pokemon_id:number
  }


  type Ability =  {
    name: String,
    url: String,
  }


  type Stat = {
    name:String,
    url:String
  }

  type Stats = {
    base_stat:number,
    effort:number,
    stat:Stat
  }

  type Abilities  = {
    ability: Ability,
    is_hidden: boolean,
    slot: number,
  }
    
  type pokemonInformation = {
    base_experience:number,
    height:number,
    weight:number,
    abilities:Object<Abilities>,
    stats:Object<Stats>
  }

  export type PokemonResult = {
    pokemon:PokemonInterface,
    pokemon_information:pokemonInformation
  }
</script>
