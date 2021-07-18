pub mod PlayerTracker
{
    use lexov_core::data_structures::{Player, PlayerList, Position};
    pub struct PlayerListContainer {
        pub playList: PlayerList,
    }
    
    impl PlayerListContainer
    {   
         pub fn new() -> PlayerListContainer {
            PlayerListContainer {  //milliseconds
                playList: lexov_core::data_structures::PlayerList {
                    players: Vec::new()
                }
        }
    }
        //remove player with UUID
        pub fn remove(& mut self, uuid: u64)
        {
            let mut i:usize =0;
            while i < self.playList.players.len() {
                if self.playList.players[i].uuid==uuid{
                    self.playList.players.remove(i);
                    break;
                    // your code here
                } else {
                    i += 1;
                }
            }
        }
        //add or update player
        pub fn update(& mut self, player: Player)
        { 
            let mut i:usize =0;
            while i < self.playList.players.len() {
                if self.playList.players[i].uuid==player.uuid{
                    self.playList.players[i].position=player.position;
                    return;
                    // your code here
                } else {
                    i += 1;
                }
            }
            self.playList.players.push(player);
        }


        
        //Quick and dirty Get a serialized PlayerList.
        pub fn getData(& self)  -> String
        {
            match serde_json::to_string(&self.playList)
            {
                Ok(v) => v,
                Err(e) => e.to_string()
            }
        }

        
        //Quick and dirty Get serialized PlayerList without a particular UUID.
        pub fn getDataExcludeUUID(& self, uuid: u64)  -> String
        {
            let mut playListClone = self.playList.clone();
            let mut i:usize =0;
            while i < playListClone.players.len() {
                if playListClone.players[i].uuid==uuid{
                    playListClone.players.remove(i);
                    break;
                    // your code here
                } else {
                    i += 1;
                }
            }
            match serde_json::to_string(&playListClone)
            {
                Ok(v) => v,
                Err(e) => e.to_string()
            }
        }
    }
    
}