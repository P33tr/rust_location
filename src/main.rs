use tokio::main;
use local_ip_address::local_ip;
use geolocation;
use std::f64::consts::PI;

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
            println!("Timezone: {:?}", info.city);

            //let lat = 40.7128;
            //let lng = -74.0060;
            // Convert strings to f64
            let lat: f64 = info.latitude.parse().expect("Invalid latitude");
            let lng: f64 = info.longitude.parse().expect("Invalid longitude");

            // Get bounding box with 2km radius
            let bbox = lat_lng_to_bbox(lat, lng, 2.0);

            // Destructure the bounding box
            let (south, west, north, east) = bbox;

            // Print individual coordinates
            println!("South: {}", south);
            println!("West: {}", west);
            println!("North: {}", north);
            println!("East: {}", east);

            // Use in Overpass API query format
            let overpass_query = format!(
                "[out:json];
        (
            node({},{},{},{});
            way({},{},{},{});
            relation({},{},{},{});
        );
        out body;
        >>;
        out skel qt;",
                south, west, north, east,
                south, west, north, east,
                south, west, north, east
            );
        } else {
            println!("Couldn't find geolocation information for IP: {:?}", ip_str);
        }
    } else {
        println!("Couldn't get an IP address");
    }

    /// Convert latitude/longitude to a bounding box
    ///
    /// # Arguments
    /// * `lat` - Center latitude
    /// * `lng` - Center longitude
    /// * `radius_km` - Radius around point in kilometers
    ///
    /// # Returns
    /// Tuple of (south, west, north, east) bounding box coordinates
    fn lat_lng_to_bbox(lat: f64, lng: f64, radius_km: f64) -> (f64, f64, f64, f64) {
        // Earth's radius in kilometers
        const EARTH_RADIUS: f64 = 6371.0;

        // Convert radius to degrees
        let lat_change = (radius_km / EARTH_RADIUS) * (180.0 / PI);
        let lng_change = lat_change / (lat.to_radians()).cos();

        // Calculate bounding box
        let south = lat - lat_change;
        let north = lat + lat_change;
        let west = lng - lng_change;
        let east = lng + lng_change;

        (south, west, north, east)
    }

    fn main() {
        // Example usage
        let center_lat = 40.7128;  // New York City latitude
        let center_lng = -74.0060; // New York City longitude
        let bbox = lat_lng_to_bbox(center_lat, center_lng, 2.0);

        println!("Overpass API Bounding Box: {:?}", bbox);
    }
}
