from openai import OpenAI
import os
import json
from pythonosc import udp_client, dispatcher, osc_server
import threading
import re
import time

#Set up Assistant
client = OpenAI(
   api_key=os.environ.get("OPENAI_API_KEY"),
)

assistant = client.beta.assistants.create(
    name="2D Graphed Image Interpreter",
    instructions="You are a interpeter of a 2D graph. Given a list of points and connections between points\
    , you are meant to interpret what basic images or shapes they might represent.",
    tools=[{"type": "retrieval"}],
    model="gpt-4-1106-preview"
)

thread = client.beta.threads.create()


#Set up server for communicating with toios
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


def wait_for_completion(run_id):
    """
    Wait for completion of prompt to continue
    """
    while(True):
        run = client.beta.threads.runs.retrieve(
			thread_id=thread.id,
			run_id=run_id
		)
        if(run["status"] == "complete"):
            return
        time.sleep(1)
          
#handle individual call
def interpret_shape(user_input, toio_positions):
    message_counter = 0
    toio_positions_str = json.dumps(toio_positions)
    if(user_input != ''):
        extra_input = f"Here is some clarifying information: {user_input}."
    input = f"Here is my list of points: {toio_positions_str}. {extra_input} What shape or basic image could this represent? "
    while(input != ''):
        client.beta.threads.messages.create(
			thread_id=thread.id,
			role="user",
			content=input,
		)
        run = client.beta.threads.runs.create(
			thread_id=thread.id,
			assistant_id=assistant.id,
		)
        wait_for_completion(run.id)
        messages = client.beta.threads.messages.list(
			thread_id=thread.id
		)
        print(messages[message_counter])
        final_result = messages[message_counter]
        print('\n\n')
        input = input("Are any of these interpretations accurate? (Enter if correct, otherwise write further instruction)")
        message_counter += 1
        
    client.beta.threads.messages.create(
		thread_id=thread.id,
		role="user",
		content="Return the final interpretation as a one or two word answer",
	)
    run = client.beta.threads.runs.create(
		thread_id=thread.id,
		assistant_id=assistant.id,
	)
    wait_for_completion(run.id)
    final_result = client.beta.threads.messages.list(
		thread_id=thread.id
	)[message_counter]
    return final_result
        
        
def main():
    disp = dispatcher.Dispatcher()
    disp.map("/test_reply", handle_test_reply)
    disp.map("/cube_positions", handle_cube_positions)

    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")

    #while True:
    user_message = input("Add further information to interpret toio positions (optional)")
    result = interpret_shape(user_message, global_cube_positions)
    print("###################################")
    final_result = result
    print(final_result)
    #try:
        #new_positions = extract_json_from_output(result['output'])
        #print(new_positions)
   	#except ValueError as e:
       # print(str(e))
    
    p#rint(new_positions)
    #client.send_message("/new_positions", str(new_positions))


if __name__ == "__main__":
    main()
