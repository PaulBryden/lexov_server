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
        pub fn update(& mut self, player: Player)
        { 
            let mut i:usize =0;
            while i < self.playList.players.len() {
                if self.playList.players[i].uuid==player.uuid{
                    self.playList.players[i].position=player.position;
                    break;
                    // your code here
                } else {
                    i += 1;
                }
            }
            self.playList.players.push(player);
        }


        
        pub fn getData(& self)  -> String
        {
            match serde_json::to_string(&self.playList)
            {
                Ok(v) => v,
                Err(e) => e.to_string()
            }
        }
    }
    
}