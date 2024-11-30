
/**
 * @TJS-additionalProperties true
*/
export type AllGameData ={
  allPlayers:Player[];
  activePlayer:ActivePlayer;
}
/**
 * @TJS-additionalProperties true
*/
type Player = {
  championName: string;
};

type ActivePlayer = {
  currentGold: number;
}
