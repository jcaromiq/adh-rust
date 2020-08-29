use crate::commands::command::Command;
use async_trait::async_trait;
pub struct RemoveImages;

#[async_trait]
impl Command for RemoveImages {
    async fn execute(&self) {
        println!("ri command");
    }
}
