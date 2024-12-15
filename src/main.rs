mod bot;
mod expre;
use bot::telegram;
mod mouse;
use mouse::mo;
mod jugada;
mod pantalla;
use pantalla::capture;
mod texto;
use texto::lector;


#[tokio::main]
async fn main(){

telegram().await;

}
