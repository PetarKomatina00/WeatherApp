use std::{collections::HashMap, sync::{Arc, atomic::{AtomicBool, AtomicI32, AtomicU32}}};

use rocket::tokio::sync::Mutex;
use tokio::sync::Notify;

//Earlier implementation was that a city has a Mutex. Every city (request) will lock the mutex.
//Problem with the earlier approach is that every other city/request(apart from the first) will by SEQUENCE get the mutex and call redis.
//Approach is ok but it can go faster.
//Newer approach is to Notify (awake) all other procceses that wait for a specific city.
//Example: 60 calls for London. Proccess A becomes a leader, 59 procceses wait.
//When Leader finishes, awake all other processes to call redis.
//Again, in the first approach proccess B calls redis(gets lock, calls redis, release lock), then process C...
//Before the first approach for 60 calls benchmark was 2.4seconds.
//After the first approach for 60 calls benchmark was 360miliseconds. LETS GO FASTER.
/* To be precise
60 responses
Success rate: 100.00%
Total:        359.7529 ms
Slowest:      357.2624 ms
Fastest:      262.6691 ms
Average:      310.4185 ms
Requests/sec: 166.7812
 */
//After the second approach for 60 calls benchmark was 270ms
/*
Success rate: 100.00%
  Total:        270.5732 ms
  Slowest:      267.9467 ms
  Fastest:      244.7302 ms
  Average:      256.7136 ms
  Requests/sec: 221.7515
*/
pub struct WeatherSingleFlight{
    flights: Mutex<HashMap<String, Arc<Flight>>>,
    pub num_leaders: Arc<AtomicU32>,
    pub num_followers: Arc<AtomicU32>
}

impl WeatherSingleFlight{
    pub fn new() -> Self{
        Self { 
            flights: Mutex::new(HashMap::new()),
            num_leaders: Arc::new(AtomicU32::new(0)),
            num_followers: Arc::new(AtomicU32::new(0))
        }
    }


    pub async fn get_lock(&self, key: &str) -> Arc<Flight>{
        let mut locks = self.flights.lock().await;

        let result = locks.entry(key.to_string())
        .or_insert_with(|| Arc::new(Flight::new()));

        result.clone()
    }

    pub async fn join(&self, key: &str) -> FlightRole{
        let mut flights: tokio::sync::MutexGuard<'_, HashMap<String, Arc<Flight>>> = self.flights.lock().await;

        if let Some(flight) = flights.get(key){
            self.num_followers.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return FlightRole::Follower(flight.clone());
        }

        let flight = Arc::new(Flight::new());

        flights.insert(key.to_string(), flight.clone());
        self.num_leaders.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        FlightRole::Leader(flight)
    }
    pub async fn remove_flight_from_hashmap(&self, key: &str, flight: &Arc<Flight>){
        let mut flights = self.flights.lock().await;
        let should_remove = flights.get(key).map(|current_flight| Arc::ptr_eq(current_flight, flight)).unwrap_or(false);

        if should_remove{
            flights.remove(key);
        }
    }
}

pub enum FlightRole{
    Leader(Arc<Flight>),
    Follower(Arc<Flight>)
}


pub struct Flight{
    notify: Notify,
    completed: AtomicBool
}

impl Flight{
    pub fn new() -> Self{
        Self { notify: Notify::new(), completed: AtomicBool::new(false) }
    }

    pub fn complete_and_notify_followers(&self){
        self.completed.store(true, std::sync::atomic::Ordering::Release);

        self.notify.notify_waiters();
    }

    pub async fn wait_for_leader(&self){
        loop{
            let notified = self.notify.notified();

            if self.completed.load(std::sync::atomic::Ordering::Acquire){
                return;
            }

            notified.await;
        }
    }
}