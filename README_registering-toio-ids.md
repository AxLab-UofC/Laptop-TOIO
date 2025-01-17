# About Registering new 'Unregistered' toios:
To employ laptop toio with manually assigned ID (as labeled below for AxLab's toios), all toios need to be 'registered' in the laptop toio file.

![IMG_3703](https://github.com/user-attachments/assets/ad4a31bc-efe0-42be-a5e9-d32650966be5)

This manually assigned ID is helpful, as toio connection is usually done by searching and automatically connecting whatever the software fines - you can't define which specific toio you want to connect to, making it diffuclt to use in classroom setting, or with sitaution that has many toios in the room. 

This document instructs the method to 'preregister' Toios in laptop toio.

NOTE: Don't follow this manual, if you are using pre-labeled toios (e.g. using AxLab's toio for Ken's Class).


## Creating and　registering toio IDs to the directory of ID aliases
1. Turn on one toio you want to register the name.
2. Navigate to the rust_osc folder, and run the command '`cargo run -- -s`. The available toio ID values (i.e. 88E) should appear in the order of connection.
3. Use this ID to update the ID aliases on rust_osc/src/main.rs under with the variable 'const IDARR'



## [Optional] You can connect to toios without using the manually assigned ID, but you can't specify which toio to connect or its order.

As follows:
1. Activate the toios in question, up to twelve at a time
2. Navigate to the rust_osc folder, and run the command '`cargo run -- -s`. The available toio ID values (i.e. 88E) should appear in the order of connection.
