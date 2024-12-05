use std::time::Duration;

use enigo::*;
use tokio::time::sleep;


pub async fn mo(){

  let mut enigo=Enigo::new(&Settings::default()).unwrap();

 loop {
     
 
  let (x,y)=enigo.location().unwrap();

  println!("x={} y={}",x,y);

  sleep(Duration::from_secs(4)).await;

 }
}