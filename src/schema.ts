type Player = {
    championName: string;
    isBot: boolean;
    isDead: boolean;
    items: any[]; 
    level: number;
    position: string;
    rawChampionName: string;
    rawSkinName: string;
    respawnTimer: number;
    riotId: string;
    riotIdGameName: string;
    riotIdTagLine: string;
    runes: {
      keystone: {
        displayName: string;
        id: number;
        rawDescription: string;
        rawDisplayName: string;
      };
      primaryRuneTree: {
        displayName: string;
        id: number;
        rawDescription: string;
        rawDisplayName: string;
      };
      secondaryRuneTree: {
        displayName: string;
        id: number;
        rawDescription: string;
        rawDisplayName: string;
      };
    };
    scores: {
      assists: number;
      creepScore: number;
      deaths: number;
      kills: number;
      wardScore: number;
    };
    skinID: number;
    skinName: string;
    summonerName: string;
    summonerSpells: {
      summonerSpellOne: {
        displayName: string;
        rawDescription: string;
        rawDisplayName: string;
      };
      summonerSpellTwo: {
        displayName: string;
        rawDescription: string;
        rawDisplayName: string;
      };
    };
    team: string;
  };
  
  type GameEvent = {
    EventID: number;
    EventName: string;
    EventTime: number;
  };
  
  type GameData = {
    gameMode: string;
    gameTime: number;
    mapName: string;
    mapNumber: number;
    mapTerrain: string;
  };
  
  type GameState = {
    allPlayers: Player[];
    events: {
      Events: GameEvent[];
    };
    gameData: GameData;
  };
  