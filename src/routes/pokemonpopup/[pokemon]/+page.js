export const load = ({params}) => {
  let pokemon_id = +params.pokemon;

  //Check of pokemon id een nummer is en tussen de 0 en 900 zit
  //>900 pokemon bestaat  niet
  //<0 pokemon bestaat niet
  if (isNaN(pokemon_id) || pokemon_id <=0 || pokemon_id >= 900) {
    return {
      error:"Pokemon doesn't exist",
    }
  }
  else{
    return {
      pokemon_id: pokemon_id,
      error:"none"
    }
  }
}
