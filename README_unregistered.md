# Connecting to Unregistered Toios:
NOTE: The steps outlined here should only be undertaken if ALL the following is true:
1. You are NOT a student taking a class taught by Prof. Ken Nakagaki
2. There is nobody nearby who is actively working with toios, as these steps may result in accidental connection to their toios

As follows:
1. Activate the toios in question, up to twelve at a time
2. Navigate to the rust_osc folder, and run the command '`cargo run -- -s`. The available toio ID values (i.e. 88E) should appear in the order of connection.
3. To connect to the toios using these internal ID values, as opposed to AxLab ID values, use the `-n` flag. For example, `cargo run -- -n P1B` or `cargo run -- -n P1B,88E,1G3`. 

## Creating and managing directory of id aliases
- For ease of use, it is best practice to label the toios with ID values. These can either be internal values (i.e. 88E) or some custom code system, such as that used by the AxLab. 
- For AxLab Members: AxLab ID "nicknames" are managed within the main.rs file. Currently, 193 toios are managed in this way.
- Organizations which are not AxLab may find it usefule to establish a similar structure, though this is not required. 