use tokio::main;
use local_ip_address::local_ip;
use geolocation;

#[tokio::main]
async fn main() {
    println!("Getting ip address!");

    let my_local_ip = local_ip();

    if let Ok(my_local_ip) = my_local_ip {
        println!("This is my local IP address: {:?}", my_local_ip);
    } else {
        println!("Error getting local IP: {:?}", my_local_ip);
    }

    for ifa in netif::up().unwrap() {
        if !ifa.address().is_loopback() {
            println!("{:?}", ifa);
        }
    }


    if let Some(ip) = public_ip::addr().await {
        let ip_str = ip.to_string();
        println!("Public IP address as string: {:?}", ip_str);

        let info = geolocation::find(&ip_str).ok();

        if let Some(info) = info {
            println!("Latitude: {:?}", info.latitude);
            println!("Longitude: {:?}", info.longitude);
            println!("Country: {:?}", info.country);
            println!("Location: {:?}", info.location);
            println!("Region: {:?}", info.region);
            println!("Timezone: {:?}", info.timezone);
        } else {
            println!("Couldn't find geolocation information for IP: {:?}", ip_str);
        }
    } else {
        println!("Couldn't get an IP address");
    }
}
