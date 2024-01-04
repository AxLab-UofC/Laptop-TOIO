from openai import OpenAI
import os
import json
from pythonosc import udp_client, dispatcher, osc_server
import threading
import re
import time

#Dummmy data
dummy_toio_positions = [{'id': 0, 'x': 0, 'y': 0, 'theta': 130},
                         {'id': 1, 'x': 1, 'y': 1, 'theta': 208},
                         {'id': 2, 'x': 1, 'y': 0, 'theta': 208},
                         {'id': 3, 'x': 0, 'y': 1, 'theta': 208}]

#Set up Assistant
gpt = OpenAI(
   api_key=os.environ.get("OPENAI_API_KEY"),
)

assistant = gpt.beta.assistants.create(
    name="2D Graphed Image Interpreter",
    instructions="You are a interpeter of a 2D graph. Given a list of points and connections between points\
    , you are meant to interpret what basic images or shapes they might represent. The 2D plane you are working with has dimensions\
    of 1000 by 1000, with 0 being in the top left corner.",
    tools=[{"type": "retrieval"}],
    model="gpt-4-1106-preview"
)

thread = gpt.beta.threads.create()


#Set up server for communicating with toios
client = udp_client.SimpleUDPClient("127.0.0.1", 4444)

global_toio_positions = []

def handle_toio_positions(unused_addr, *args):
    global global_toio_positions
    global_toio_positions.clear()
    global_toio_positions = [{'id': args[i], 'x': args[i+1], 'y': args[i+2], 'theta': args[i+3]} for i in range(0, len(args), 4)]

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

          
#handle individual call
def wait_for_completion(run_id):
    """
    Wait for completion of prompt to continue
    """
    while(True):
        run = gpt.beta.threads.runs.retrieve(
			thread_id=thread.id,
			run_id=run_id
		)
        if(run.status == "completed"):
            return
        time.sleep(3)
        print("Loading...")
	
	
def interpret_toios(user_input, toio_positions):
    """
    Interpret toios with help from user
    """
    message_counter = 0
    toio_positions_str = json.dumps(dummy_toio_positions)
    extra_input = ''
    if(user_input != ''):
        extra_input = f"Here is some clarifying information: {user_input}."
    prompt = f"Here is my list of points: {toio_positions_str}. {extra_input} What shape or basic image could this represent? Give your answer in one or two sentences."
    while(prompt != ''):
        gpt.beta.threads.messages.create(
			thread_id=thread.id,
			role="user",
			content=prompt,
		)
        message_counter += 1
        run = gpt.beta.threads.runs.create(
			thread_id=thread.id,
			assistant_id=assistant.id,
		)
        wait_for_completion(run.id)
        messages = gpt.beta.threads.messages.list(
			thread_id=thread.id
		)
        mes = messages.data[0].content[0].text.value
        print(mes)
        final_result = mes
        print('\n')
        message_counter += 1
        prompt = input("Are any of these interpretations accurate? (Enter if correct, otherwise write further instruction): ")
        
    gpt.beta.threads.messages.create(
		thread_id=thread.id,
		role="user",
		content="Return the final interpretation as a one or two word answer",
	)
    message_counter += 1
    run = gpt.beta.threads.runs.create(
		thread_id=thread.id,
		assistant_id=assistant.id,
	)
    wait_for_completion(run.id)
    messages = gpt.beta.threads.messages.list(
		thread_id=thread.id
	)
    final_result = messages.data[0].content[0].text.value
    return final_result


def new_movements(interpretation, toio_positions):
    """
    Generate new positions given interpretation
    """
    gpt.beta.threads.messages.create(
		thread_id=thread.id,
		role="user",
		content="Return the final interpretation as a one or two word answer",
	)

 
def main():
    disp = dispatcher.Dispatcher()
    disp.map("/test_reply", handle_test_reply)
    disp.map("/cube_positions", handle_toio_positions)

    server = osc_server.ThreadingOSCUDPServer(("127.0.0.1", 4445), disp)

    # Use threading to start the OSC server without blocking
    server_thread = threading.Thread(target=server.serve_forever)
    server_thread.start()

    print("Listening for OSC messages on port 4445...")

    #while True:
    user_message = input("Add further information to interpret toio positions (optional): ")
    result = interpret_toios(user_message, global_toio_positions)
    print("###################################")
    final_result = result
    print(final_result)
    #try:
        #new_positions = extract_json_from_output(result['output'])
        #print(new_positions)
   	#except ValueError as e:
       # print(str(e))
    
    #print(new_positions)
    #client.send_message("/new_positions", str(new_positions))


if __name__ == "__main__":
    main()
