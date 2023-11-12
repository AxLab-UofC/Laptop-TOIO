from pythonosc import udp_client, dispatcher, osc_server
import threading
from Agent import invoke_agent
import re
import json


client = udp_client.SimpleUDPClient("127.0.0.1", 4444)

global_cube_positions = []

def handle_cube_positions(unused_addr, *args):
    global global_cube_positions
    global_cube_positions.clear()
    global_cube_positions = [{'id': args[i], 'x': args[i+1], 'y': args[i+2], 'theta': args[i+3]} for i in range(0, len(args), 4)]

def handle_test_reply(unused_addr, message):
    """Handle the test_reply message from Processing."""
    print("Received from Processing: " + message)

def extract_json_from_output(output):
    # Regular expression to match a JSON list
    json_list_pattern = r'\[.*?\]'
    matches = re.findall(json_list_pattern, output)
    if matches:
        # Assuming the first match is the JSON list you need
        json_list = matches[0]
        return json.loads(json_list)  # Convert JSON string to Python list
    else:
        raise ValueError("No JSON list found in the output")

def main():
    disp = dispatcher.Dispatcher()
    disp.map("/test_reply", handle_test_reply)
    disp.map("/cube_positions", handle_cube_positions)

    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")

    while True:
        user_message = input("Enter your Prompt: ")
        result = invoke_agent(user_message, global_cube_positions)
        print("###################################")
        final_result = result["output"]
        print(final_result)
        try:
            new_positions = extract_json_from_output(result['output'])
            print(new_positions)
        except ValueError as e:
            print(str(e))
        
        print(new_positions)
        client.send_message("/new_positions", str(new_positions))

if __name__ == "__main__":
    main()
