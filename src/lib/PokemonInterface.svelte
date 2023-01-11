<script lang="ts" context="module">
  export type PokemonInterface = {
    id:number,
    name:string, 
    sound:String,
    image:String 
  }
  
  export type AllPokemon = {
    all_pokemon:[PokemonInterface]   
  }
</script>


