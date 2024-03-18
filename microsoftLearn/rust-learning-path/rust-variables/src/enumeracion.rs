/*enum WebEvent {
    // An enum variant can be like a unit struct without fields or data types
    WELoad,
    // An enum variant can be like a tuple struct with data types but no named fields
    WEKeys(String, char),
    // An enum variant can be like a classic struct with named fields and their data types
    WEClick { x: i64, y: i64 },
}
*/

// Define a tuple struct
struct KeyPress(String, char);

// Define a classic struct
struct MouseClick {
    x: i64,
    y: i64,
}

// Redefine the enum variants to use the data from the new structs
// Update the page Load variant to have the boolean type
enum WebEvent {
    WELoad(bool),
    WEKeys(KeyPress),
    WEClick(MouseClick),
}

fn main() {
    let we_load = WebEvent::WELoad(true);

    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');

    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);
}
