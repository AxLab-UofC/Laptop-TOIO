from pythonosc import udp_client, dispatcher, osc_server
import threading
from Agent import invoke_agent


client = udp_client.SimpleUDPClient("127.0.0.1", 4444)

global_cube_positions = []

def handle_cube_positions(unused_addr, *args):
    global global_cube_positions  # Declare the use of the global variable
    # Clear the previous positions
    global_cube_positions.clear()
    # Process the incoming arguments and update the global variable
    global_cube_positions = [{'id': args[i], 'x': args[i+1], 'y': args[i+2], 'theta': args[i+3]} for i in range(0, len(args), 4)]
    #print(global_cube_positions)  # For debugging

def handle_test_reply(unused_addr, message):
    """Handle the test_reply message from Processing."""
    print("Received from Processing: " + message)

def main():
    # Set up the dispatcher to handle OSC messages
    disp = dispatcher.Dispatcher()
    disp.map("/test_reply", handle_test_reply)
    disp.map("/cube_positions", handle_cube_positions)

    # Set up the OSC server to listen for messages
    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")

    #LLM Agent
    while True:
        user_message = input("Enter your Prompt: ")
        result = invoke_agent(user_message, global_cube_positions)
        print("###################################")
        final_result = result.return_values["output"]
        print(final_result)
        #client.send_message("/user_input", [user_message])

if __name__ == "__main__":
    main()
